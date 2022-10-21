import { Typography } from '@material-ui/core';
import React from 'react';

export const StatusConnected: React.FC = () => (
    <>
        <br />
        <br />
        <Typography variant="h5" component="h2" gutterBottom color="primary">
            Connected
        </Typography>
    </>
);

export const StatusConnecting: React.FC = () => (
    <>
        <br />
        <br />
        <Typography variant="h5" component="h2" gutterBottom color="textSecondary">
            Connecting...
        </Typography>
    </>
);
