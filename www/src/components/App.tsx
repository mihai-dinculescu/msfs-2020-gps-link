import React from 'react';
import { Container, Box, Typography } from '@mui/material';

import { Version } from './Version';
import { ConnectForm } from './ConnectForm';

export const App: React.FC = () => (
    <>
        <Container>
            <Box height="100vh" display="flex" flexDirection="column" justifyContent="space-between">
                <Box my={2}>
                    <Typography variant="h4" component="h1" gutterBottom>
                        MSFS 2020 GPS Link
                    </Typography>
                    <Typography>Transmit GPS data from Microsoft Flight Simulator 2020 to navigation apps.</Typography>
                </Box>
                <ConnectForm />
                <Box my={2}>
                    <Version />
                </Box>
            </Box>
        </Container>
    </>
);
