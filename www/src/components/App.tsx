import React from 'react';
import { Container, Box, Typography, makeStyles } from '@material-ui/core';

import { Version } from './Version';
import { ConnectForm } from './ConnectForm';

export const useStyles = makeStyles((theme) => ({
    root: {
        '& > *': {
            margin: theme.spacing(0.5),
        },
    },
}));

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
                <ConnectForm boxClassName={classes.root} />
                <Box my={4} className={classes.root}>
                    <Version />
                </Box>
            </Container>
        </>
    );
};
