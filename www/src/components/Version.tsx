import { Alert } from '@mui/material';
import React from 'react';

const PACKAGE_VERSION = process.env.REACT_APP_VERSION?.trim();
const LATEST_VERSION_URL = 'https://raw.githubusercontent.com/mihai-dinculescu/msfs-2020-gps-link/main/version.txt';

export const Version: React.FC = () => {
    const [latestVersion, setLatestVersion] = React.useState<string | undefined>();

    React.useEffect(() => {
        const fn = async () => {
            const req = await fetch(LATEST_VERSION_URL);
            const value = await req.text();
            setLatestVersion(value.trim());
        };

        fn();
    }, []);

    return PACKAGE_VERSION && latestVersion ? (
        PACKAGE_VERSION >= latestVersion ? (
            <Alert severity="info">Version {PACKAGE_VERSION}</Alert>
        ) : (
            <Alert severity="warning">
                You are using version {PACKAGE_VERSION}.
                <br />
                <br />
                {latestVersion} is available. You can download it from:
                <br />
                https://github.com/mihai-dinculescu/msfs-2020-gps-link/releases
            </Alert>
        )
    ) : null;
};
