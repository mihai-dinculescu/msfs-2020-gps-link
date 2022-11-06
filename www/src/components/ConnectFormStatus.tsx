import { Box, Button, Stack } from '@mui/material';
import { useCallback, useContext, useEffect, useMemo } from 'react';
import { v4 as uuidv4 } from 'uuid';
import { invoke } from '@tauri-apps/api/tauri';

import { StatusConnected, StatusConnecting, StatusNone } from './Status';
import { ContactFormContext } from './ConnectForm';

const INTERVAL_CONNECTING_MS = 3 * 1000;
const INTERVAL_STATUS_CONNECTING_MS = 0.5 * 1000;
const INTERVAL_STATUS_CONNECTED_MS = 3 * 1000;

export const ConnectFormStatus: React.FC = () => {
    const {
        refreshRate,
        broadcastOver,
        udpPort,
        udpNetmask,
        comPort,
        comBaudRate,
        connectionStatus,
        setConnectionStatus,
        isDisabled,
    } = useContext(ContactFormContext);

    const connect = useCallback(() => {
        const config =
            broadcastOver === 'udp'
                ? {
                      type: 'udp',
                      port: udpPort,
                      netmask: udpNetmask,
                  }
                : {
                      type: 'com',
                      port: comPort,
                      baudRate: comBaudRate,
                  };

        invoke('cmd_start', {
            requestId: uuidv4(),
            options: {
                refreshRate,
                config,
            },
        }).catch((error) => {
            console.error('Start', error);
        });
    }, [refreshRate, broadcastOver, udpNetmask, udpPort, comPort, comBaudRate]);

    const getStatus = useCallback(() => {
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
    }, [connectionStatus.isConnecting, connectionStatus.isConnected, setConnectionStatus]);

    const onStart = useCallback(() => {
        setConnectionStatus({
            isConnecting: true,
            isConnected: false,
        });
        connect();
    }, [setConnectionStatus, connect]);

    const onStop = useCallback(() => {
        setConnectionStatus({
            isConnecting: false,
            isConnected: false,
        });

        invoke('cmd_stop', {
            requestId: uuidv4(),
        }).catch((error) => {
            console.error('Stop', error);
        });
    }, [setConnectionStatus]);

    const status = useMemo(() => {
        if (connectionStatus.isConnected) {
            return <StatusConnected />;
        } else if (connectionStatus.isConnecting) {
            return <StatusConnecting />;
        }

        return <StatusNone />;
    }, [connectionStatus.isConnecting, connectionStatus.isConnected]);

    useEffect(() => {
        if (connectionStatus.isConnecting && !connectionStatus.isConnected) {
            const timer = setInterval(connect, INTERVAL_CONNECTING_MS);
            return () => clearInterval(timer);
        }
    }, [connectionStatus.isConnecting, connectionStatus.isConnected, connect]);

    useEffect(() => {
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

    return (
        <Box my={2}>
            <Stack spacing={2} direction="row">
                <Button onClick={onStart} variant="contained" color="primary" disabled={isDisabled}>
                    Connect
                </Button>
                <Button onClick={onStop} variant="contained" color="secondary" disabled={!isDisabled}>
                    Disconnect
                </Button>
            </Stack>
            {status}
        </Box>
    );
};
