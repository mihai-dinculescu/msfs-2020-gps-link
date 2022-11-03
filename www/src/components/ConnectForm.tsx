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

import { invoke } from '@tauri-apps/api/tauri';

import { IPAddressTextMask } from './IPAddressTextMask';
import { StatusConnected, StatusConnecting } from './Status';

const INTERVAL_CONNECTING_MS = 3 * 1000;
const INTERVAL_STATUS_CONNECTING_MS = 0.5 * 1000;
const INTERVAL_STATUS_CONNECTED_MS = 3 * 1000;

interface ConnectFormProps {
    boxClassName: string;
}

export const ConnectForm: React.FC<ConnectFormProps> = (props: ConnectFormProps) => {
    const [refreshRate, setRefreshRate] = React.useState('Fast');
    const [broadcastPort, setBroadcastPort] = React.useState(49002);
    const [broadcastNetmask, setBroadcastNetmask] = React.useState('255.255.255.255');

    const [connectionStatus, setConnectionStatus] = React.useState({
        isConnecting: false,
        isConnected: false,
    });

    const getAvailableComPorts = React.useCallback(() => {
        invoke('cmd_get_available_com_ports', {
            requestId: uuidv4(),
        })
            .then((response) => {
                console.log(response);
            })
            .catch((error) => {
                console.error('Start', error);
            });
    }, []);

    const connect = React.useCallback(() => {
        invoke('cmd_start', {
            requestId: uuidv4(),
            options: {
                refreshRate,
                // config: {
                //     type: 'udp',
                //     port: broadcastPort,
                //     netmask: broadcastNetmask,
                // },
                config: {
                    type: 'com',
                    port: 'COM2',
                    baud_rate: 9600,
                },
            },
        }).catch((error) => {
            console.error('Start', error);
        });
    }, [broadcastNetmask, broadcastPort, refreshRate]);

    const getStatus = React.useCallback(() => {
        if (connectionStatus.isConnecting || connectionStatus.isConnected) {
            invoke('cmd_get_status', {
                requestId: uuidv4(),
            })
                .then((response) => {
                    const { data } = response as { data: boolean };

                    setConnectionStatus((prevState) => ({
                        ...prevState,
                        isConnected: data,
                    }));
                })
                .catch((error) => {
                    console.error('Status', error);
                });
        }
    }, [connectionStatus.isConnecting, connectionStatus.isConnected]);

    React.useEffect(() => {
        getAvailableComPorts();
    }, []);

    React.useEffect(() => {
        if (connectionStatus.isConnecting && !connectionStatus.isConnected) {
            const timer = setInterval(connect, INTERVAL_CONNECTING_MS);
            return () => clearInterval(timer);
        }
    }, [connectionStatus.isConnecting, connectionStatus.isConnected, connect]);

    React.useEffect(() => {
        let timer: NodeJS.Timer | undefined = undefined;

        if (connectionStatus.isConnecting && !connectionStatus.isConnected) {
            timer = setInterval(getStatus, INTERVAL_STATUS_CONNECTING_MS);
        } else if (connectionStatus.isConnected) {
            timer = setInterval(getStatus, INTERVAL_STATUS_CONNECTED_MS);
        }

        return () => {
            if (timer !== undefined) {
                clearInterval(timer);
            }
        };
    }, [connectionStatus.isConnecting, connectionStatus.isConnected, getStatus]);

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
        setConnectionStatus({
            isConnecting: true,
            isConnected: false,
        });
        connect();
    };

    const onStop = () => {
        setConnectionStatus({
            isConnecting: false,
            isConnected: false,
        });

        invoke('cmd_stop', {
            requestId: uuidv4(),
        }).catch((error) => {
            console.error('Stop', error);
        });
    };

    const isDisabled = connectionStatus.isConnecting || connectionStatus.isConnected;

    let status = null;

    if (connectionStatus.isConnected) {
        status = <StatusConnected />;
    } else if (connectionStatus.isConnecting) {
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
