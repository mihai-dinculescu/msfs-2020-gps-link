import Button from '@material-ui/core/Button';
import Container from '@material-ui/core/Container';
import Box from '@material-ui/core/Box';
import Typography from '@material-ui/core/Typography';
import { makeStyles } from '@material-ui/core/styles';
import Radio from '@material-ui/core/Radio';
import RadioGroup from '@material-ui/core/RadioGroup';
import FormControlLabel from '@material-ui/core/FormControlLabel';
import FormControl from '@material-ui/core/FormControl';
import FormLabel from '@material-ui/core/FormLabel';
import TextField from '@material-ui/core/TextField';
import Input from '@material-ui/core/Input';
import MaskedInput from 'react-text-mask';

import { v4 as uuidv4 } from 'uuid';
import { promisified } from 'tauri/api/tauri';
import React from 'react';

const useStyles = makeStyles((theme) => ({
    root: {
        '& > *': {
            margin: theme.spacing(0.5),
        },
    },
}));

interface TextMaskCustomProps {
    inputRef: (ref: HTMLInputElement | null) => void;
}

const TextMaskCustom = (props: TextMaskCustomProps) => {
    const { inputRef, ...other } = props;

    return (
        <MaskedInput
            {...other}
            ref={(ref: any) => {
                inputRef(ref ? ref.inputElement : null);
            }}
            mask={[/\d/, /\d/, /\d/, '.', /\d/, /\d/, /\d/, '.', /\d/, /\d/, /\d/, '.', /\d/, /\d/, /\d/]}
            placeholderChar={'\u2000'}
            showMask
        />
    );
};

export const App = () => {
    const classes = useStyles();
    const [refreshRate, setRefreshRate] = React.useState('Fast');
    const [broadcastPort, setBroadcastPort] = React.useState(49002);
    const [broadcastNetmask, setBroadcastNetmask] = React.useState('255.255.255.255');
    const [isConnecting, setIsConnecting] = React.useState(false);
    const [isConnected, setIsConnected] = React.useState(false);

    const connect = () => {
        promisified({
            cmd: 'start',
            requestId: uuidv4(),
            options: {
                broadcastNetmask,
                broadcastPort,
                refreshRate,
            },
        })
            .then((response: any) => {
                // do something with the Ok() response
                // const { message } = response;
            })
            .catch((error) => {
                // do something with the Err() response string
                console.error('Start', error);
            });
    };

    const getStatus = () => {
        if (isConnecting || isConnected) {
            promisified({
                cmd: 'status',
                requestId: uuidv4(),
            })
                .then((response: any) => {
                    // do something with the Ok() response
                    const { message } = response;

                    if (message === 'OK') {
                        setIsConnected(true);
                    } else {
                        setIsConnected(false);
                    }
                })
                .catch((error) => {
                    // do something with the Err() response string
                    console.error('Status', error);
                });
        }
    };

    React.useEffect(() => {
        if (isConnecting && !isConnected) {
            const timer = setInterval(connect, 3 * 1000);
            return () => clearInterval(timer);
        }
    });

    React.useEffect(() => {
        const timer = setInterval(getStatus, 3 * 1000);
        return () => clearInterval(timer);
    });

    const refreshRateOnChange = (event: any) => {
        setRefreshRate(event.target.value);
    };

    const broadcastPortOnChange = (event: any) => {
        setBroadcastPort(parseInt(event.target.value, 10));
    };

    const broadcastNetmaskOnChange = (event: any) => {
        setBroadcastNetmask(event.target.value);
    };

    const onStart = () => {
        setIsConnecting(true);
        setIsConnected(false);
        connect();
    };

    const onStop = () => {
        setIsConnecting(false);
        setIsConnected(false);

        promisified({
            cmd: 'stop',
            requestId: uuidv4(),
        })
            .then((response: any) => {
                // do something with the Ok() response
                const { message } = response;
                console.log(message);
            })
            .catch((error) => {
                // do something with the Err() response string
                console.error(error);
            });
    };

    const isDisabled = isConnecting || isConnected;

    let status = null;

    if (isConnected) {
        status = (
            <Typography variant="h5" component="h2" gutterBottom color="primary">
                Connected
            </Typography>
        );
    } else if (isConnecting) {
        status = (
            <Typography variant="h5" component="h2" gutterBottom color="textSecondary">
                Connecting...
            </Typography>
        );
    }

    return (
        <>
            <Container maxWidth="md">
                <Box my={4} className={classes.root}>
                    <Typography variant="h4" component="h1" gutterBottom>
                        MSFS 2020 GPS Link
                    </Typography>
                    <Typography>Transmit GPS data from Microsoft Flight Simulator 2020 to navigation apps.</Typography>
                    <Typography>
                        Tested with{' '}
                        <a target="_bank" href="https://www.skydemon.aero/">
                            SkyDemon
                        </a>{' '}
                        and{' '}
                        <a target="_bank" href="https://buy.garmin.com/en-US/US/p/115856">
                            Garmin Pilot
                        </a>
                        .
                    </Typography>
                    <br />
                    <FormControl component="fieldset">
                        <FormLabel component="legend">Refresh rate</FormLabel>
                        <RadioGroup
                            aria-label="Refresh Rate"
                            name="refreshRate"
                            value={refreshRate}
                            onChange={refreshRateOnChange}
                        >
                            <FormControlLabel
                                value="Fast"
                                control={<Radio disabled={isDisabled} />}
                                label="Fast (~ten times a second)"
                            />
                            <FormControlLabel
                                value="Slow"
                                control={<Radio disabled={isDisabled} />}
                                label="Slow (once a second)"
                            />
                        </RadioGroup>
                    </FormControl>
                    <FormControl component="fieldset">
                        <FormLabel component="legend">Broadcast subnet mask</FormLabel>
                        <Input
                            disabled={isDisabled}
                            name="broadcastNetmask"
                            id="broadcastNetmask"
                            inputComponent={TextMaskCustom as any}
                            value={broadcastNetmask}
                            onChange={broadcastNetmaskOnChange}
                        />
                    </FormControl>
                    <FormControl component="fieldset">
                        <FormLabel component="legend">Broadcast port</FormLabel>
                        <TextField
                            disabled={isDisabled}
                            name="broadcastPort"
                            id="broadcastPort"
                            type="number"
                            InputLabelProps={{
                                shrink: true,
                            }}
                            value={broadcastPort}
                            onChange={broadcastPortOnChange}
                        />
                    </FormControl>
                    <br />
                    <br />
                    <Button onClick={onStart} variant="contained" color="primary" disabled={isDisabled}>
                        Connect
                    </Button>
                    <Button onClick={onStop} variant="contained" color="secondary" disabled={!isDisabled}>
                        Disconnect
                    </Button>
                    <br />
                    <br />
                    {status}
                </Box>
            </Container>
        </>
    );
};
