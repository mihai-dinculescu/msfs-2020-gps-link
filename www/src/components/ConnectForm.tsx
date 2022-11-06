import { Box, Grid } from '@mui/material';
import React, { createContext, useMemo, useState } from 'react';

import { ConnectFormStatus } from './ConnectFormStatus';
import { ConnectFormRefreshRate } from './ConnectFormRefreshRate';
import { ConnectFormBroadcast } from './ConnectFormBroadcast';

export const ConnectForm: React.FC = () => (
    <>
        <ContextProvider>
            <Box>
                <Grid container>
                    <Grid item xs={5}>
                        <ConnectFormRefreshRate />
                        <Box mb={12}></Box>
                    </Grid>
                    <Grid item xs={7}>
                        <ConnectFormBroadcast />
                    </Grid>
                </Grid>
            </Box>
            <ConnectFormStatus />
        </ContextProvider>
    </>
);

export interface IConnectionStatus {
    isConnecting: boolean;
    isConnected: boolean;
}

interface IContactFormContext {
    refreshRate: string;
    setRefreshRate: React.Dispatch<React.SetStateAction<string>>;
    broadcastOver: string;
    setBroadcastOver: React.Dispatch<React.SetStateAction<string>>;
    udpPort: number;
    setUdpPort: React.Dispatch<React.SetStateAction<number>>;
    udpNetmask: string;
    setUdpNetmask: React.Dispatch<React.SetStateAction<string>>;
    comPort: string;
    setComPort: React.Dispatch<React.SetStateAction<string>>;
    comBaudRate: number;
    setComBaudRate: React.Dispatch<React.SetStateAction<number>>;
    connectionStatus: IConnectionStatus;
    setConnectionStatus: React.Dispatch<React.SetStateAction<IConnectionStatus>>;
    isDisabled: boolean;
}

export const ContactFormContext = createContext<IContactFormContext>({} as IContactFormContext);

const ContextProvider = ({ children }: { children: Array<JSX.Element> }) => {
    const [refreshRate, setRefreshRate] = useState('fast');
    const [broadcastOver, setBroadcastOver] = useState('udp');
    const [udpPort, setUdpPort] = useState(49002);
    const [udpNetmask, setUdpNetmask] = useState('255.255.255.255');
    const [comPort, setComPort] = useState('');
    const [comBaudRate, setComBaudRate] = useState(9600);

    const [connectionStatus, setConnectionStatus] = useState({
        isConnecting: false,
        isConnected: false,
    });

    const isDisabled = useMemo(
        () => connectionStatus.isConnecting || connectionStatus.isConnected,
        [connectionStatus.isConnecting, connectionStatus.isConnected],
    );

    return (
        <ContactFormContext.Provider
            value={{
                refreshRate,
                setRefreshRate,
                broadcastOver,
                setBroadcastOver,
                udpPort,
                setUdpPort,
                udpNetmask,
                setUdpNetmask,
                comPort,
                setComPort,
                comBaudRate,
                setComBaudRate,
                connectionStatus,
                setConnectionStatus,
                isDisabled,
            }}
        >
            {children}
        </ContactFormContext.Provider>
    );
};
