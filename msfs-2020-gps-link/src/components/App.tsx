import React from 'react';
import { Container, Box, Typography } from '@material-ui/core';

import { Version } from './Version';
import { useStyles } from '../hooks/useStyles';
import { ConnectForm } from './ConnectForm';

export const App: React.FC = () => {
    const classes = useStyles();

    return (
        <>
            <Container maxWidth="md">
                <Box my={4} className={classes.root}>
                    <Typography variant="h4" component="h1" gutterBottom>
                        MSFS 2020 GPS Link
                    </Typography>
                    <Typography>Transmit GPS data from Microsoft Flight Simulator 2020 to navigation apps.</Typography>
                </Box>
                <ConnectForm />
                <Box my={4} className={classes.root}>
                    <Version />
                </Box>
            </Container>
        </>
    );
};
