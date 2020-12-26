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
import Alert from '@material-ui/lab/Alert';
import MaskedInput from 'react-text-mask';

import { v4 as uuidv4 } from 'uuid';
import { promisified } from 'tauri/api/tauri';
import React from 'react';

const packageVersion = process.env.REACT_APP_VERSION;

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
            /* eslint-disable-next-line @typescript-eslint/no-explicit-any */
            ref={(ref: any) => {
                inputRef(ref ? ref.inputElement : null);
            }}
            mask={[/\d/, /\d/, /\d/, '.', /\d/, /\d/, /\d/, '.', /\d/, /\d/, /\d/, '.', /\d/, /\d/, /\d/]}
            placeholderChar={'\u2000'}
            showMask
        />
    );
};

export const App: React.FC = () => {
    const classes = useStyles();
    const [refreshRate, setRefreshRate] = React.useState('Fast');
    const [broadcastPort, setBroadcastPort] = React.useState(49002);
    const [broadcastNetmask, setBroadcastNetmask] = React.useState('255.255.255.255');
    const [isConnecting, setIsConnecting] = React.useState(false);
    const [isConnected, setIsConnected] = React.useState(false);
    const [latestVersion, setLatestVersion] = React.useState<string | undefined>();

    const connect = React.useCallback(() => {
        promisified({
            cmd: 'start',
            requestId: uuidv4(),
            options: {
                broadcastNetmask,
                broadcastPort,
                refreshRate,
            },
        }).catch((error) => {
            console.error('Start', error);
        });
    }, [broadcastNetmask, broadcastPort, refreshRate]);

    const getStatus = React.useCallback(() => {
        if (isConnecting || isConnected) {
            promisified({
                cmd: 'status',
                requestId: uuidv4(),
            })
                .then((response) => {
                    const { message } = response as { message: string };

                    if (message === 'OK') {
                        setIsConnected(true);
                    } else {
                        setIsConnected(false);
                    }
                })
                .catch((error) => {
                    console.error('Status', error);
                });
        }
    }, [isConnecting, isConnected]);

    React.useEffect(() => {
        const fn = async () => {
            const url = 'https://raw.githubusercontent.com/mihai-dinculescu/msfs-2020-gps-link/main/version.txt';

            const req = await fetch(url);
            const value = await req.text();
            setLatestVersion(value);
        };

        fn();
    }, []);

    React.useEffect(() => {
        if (isConnecting && !isConnected) {
            const timer = setInterval(connect, 3 * 1000);
            return () => clearInterval(timer);
        }
    }, [isConnecting, isConnected, connect]);

    React.useEffect(() => {
        const timer = setInterval(getStatus, 3 * 1000);
        return () => clearInterval(timer);
    }, [getStatus]);

    const refreshRateOnChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setRefreshRate(event.target.value);
    };

    const broadcastPortOnChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setBroadcastPort(parseInt(event.target.value, 10));
    };

    const broadcastNetmaskOnChange = (event: React.ChangeEvent<HTMLInputElement>) => {
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
            .then((response) => {
                const { message } = response as { message: string };
                console.log(message);
            })
            .catch((error) => {
                console.error(error);
            });
    };

    const isDisabled = isConnecting || isConnected;

    let status = null;

    if (isConnected) {
        status = (
            <>
                <br />
                <br />
                <Typography variant="h5" component="h2" gutterBottom color="primary">
                    Connected
                </Typography>
            </>
        );
    } else if (isConnecting) {
        status = (
            <>
                <br />
                <br />
                <Typography variant="h5" component="h2" gutterBottom color="textSecondary">
                    Connecting...
                </Typography>
            </>
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
                </Box>
                <Box my={4} className={classes.root}>
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
                            /* eslint-disable-next-line @typescript-eslint/no-explicit-any */
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
                </Box>
                <Box my={4} className={classes.root}>
                    <Button onClick={onStart} variant="contained" color="primary" disabled={isDisabled}>
                        Connect
                    </Button>
                    <Button onClick={onStop} variant="contained" color="secondary" disabled={!isDisabled}>
                        Disconnect
                    </Button>
                    {status}
                </Box>
                <Box my={4} className={classes.root}>
                    {latestVersion ? (
                        packageVersion?.trim() === latestVersion?.trim() ? (
                            <Alert severity="info">Version {packageVersion}</Alert>
                        ) : (
                            <Alert severity="warning">
                                There is a new version available. Download it from:
                                <br />
                                https://github.com/mihai-dinculescu/msfs-2020-gps-link/releases
                            </Alert>
                        )
                    ) : null}
                </Box>
            </Container>
        </>
    );
};
