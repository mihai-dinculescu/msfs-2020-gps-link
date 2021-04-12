import {
    Box,
    FormControl,
    FormLabel,
    RadioGroup,
    FormControlLabel,
    Radio,
    Input,
    TextField,
    Button,
    Tooltip,
    Typography,
} from '@material-ui/core';
import HelpIcon from '@material-ui/icons/Help';
import React from 'react';
import { v4 as uuidv4 } from 'uuid';

import { promisified } from 'tauri/api/tauri';

import { IPAddressTextMask } from './IPAddressTextMask';
import { StatusConnected, StatusConnecting } from './Status';

interface ConnectFormProps {
    boxClassName: string;
}

export const ConnectForm: React.FC<ConnectFormProps> = (props: ConnectFormProps) => {
    const [refreshRate, setRefreshRate] = React.useState('Fast');
    const [broadcastPort, setBroadcastPort] = React.useState(49002);
    const [broadcastNetmask, setBroadcastNetmask] = React.useState('255.255.255.255');
    const [isConnecting, setIsConnecting] = React.useState(false);
    const [isConnected, setIsConnected] = React.useState(false);

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
        status = <StatusConnected />;
    } else if (isConnecting) {
        status = <StatusConnecting />;
    }

    return (
        <>
            <Box my={4} className={props.boxClassName}>
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
                    <FormLabel component="legend">Broadcast address</FormLabel>
                    <Input
                        disabled={isDisabled}
                        name="broadcastNetmask"
                        id="broadcastNetmask"
                        inputComponent={IPAddressTextMask}
                        value={broadcastNetmask}
                        onChange={broadcastNetmaskOnChange}
                        endAdornment={
                            <Tooltip
                                title={
                                    <>
                                        <Typography>Use either</Typography>
                                        <Typography>
                                            - a netmask like 255.255.255.255 if you want to broadcast to your entire
                                            network
                                        </Typography>
                                        <Typography>
                                            - the exact IP address of the device you plan to use for navigation
                                        </Typography>
                                    </>
                                }
                            >
                                <HelpIcon color={isDisabled ? 'disabled' : 'primary'} />
                            </Tooltip>
                        }
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
            <Box my={4} className={props.boxClassName}>
                <Button onClick={onStart} variant="contained" color="primary" disabled={isDisabled}>
                    Connect
                </Button>
                <Button onClick={onStop} variant="contained" color="secondary" disabled={!isDisabled}>
                    Disconnect
                </Button>
                {status}
            </Box>
        </>
    );
};
