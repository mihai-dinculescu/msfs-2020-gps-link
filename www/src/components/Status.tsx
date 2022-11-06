import { Typography, Box } from '@mui/material';
import React from 'react';

export const StatusNone: React.FC = () => (
    <Box my={2}>
        <Typography variant="h5" component="h2" gutterBottom color="primary">
            &nbsp;
        </Typography>
    </Box>
);

export const StatusConnected: React.FC = () => (
    <Box my={2}>
        <Typography variant="h5" component="h2" gutterBottom color="primary">
            Connected
        </Typography>
    </Box>
);

export const StatusConnecting: React.FC = () => (
    <Box my={2}>
        <Typography variant="h5" component="h2" gutterBottom color="textSecondary">
            Connecting...
        </Typography>
    </Box>
);
