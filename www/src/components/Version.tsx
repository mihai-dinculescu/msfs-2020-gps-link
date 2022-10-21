import { Alert } from '@material-ui/lab';
import React from 'react';

const PACKAGE_VERSION = process.env.REACT_APP_VERSION;
const LATEST_VERSION_URL = 'https://raw.githubusercontent.com/mihai-dinculescu/msfs-2020-gps-link/main/version.txt';

export const Version: React.FC = () => {
    const [latestVersion, setLatestVersion] = React.useState<string | undefined>();

    React.useEffect(() => {
        const fn = async () => {
            const req = await fetch(LATEST_VERSION_URL);
            const value = await req.text();
            setLatestVersion(value);
        };

        fn();
    }, []);

    return latestVersion ? (
        PACKAGE_VERSION?.trim() === latestVersion?.trim() ? (
            <Alert severity="info">Version {PACKAGE_VERSION}</Alert>
        ) : (
            <Alert severity="warning">
                There is a new version available. Download it from:
                <br />
                https://github.com/mihai-dinculescu/msfs-2020-gps-link/releases
            </Alert>
        )
    ) : null;
};
