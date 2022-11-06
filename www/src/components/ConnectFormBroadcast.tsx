import { Help } from '@mui/icons-material';
import {
    Box,
    FormControl,
    FormControlLabel,
    FormLabel,
    Input,
    MenuItem,
    Radio,
    RadioGroup,
    Select,
    SelectChangeEvent,
    Stack,
    TextField,
    Tooltip,
    Typography,
} from '@mui/material';
import { useCallback, useContext, useEffect, useMemo, useState } from 'react';
import { v4 as uuidv4 } from 'uuid';
import { invoke } from '@tauri-apps/api/tauri';

import { IPAddressTextMask } from './IPAddressTextMask';
import { ContactFormContext } from './ConnectForm';

export const ConnectFormBroadcast: React.FC = () => {
    const [availableComPorts, setAvailableComPorts] = useState<string[]>([]);

    const {
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
        isDisabled,
    } = useContext(ContactFormContext);

    const getAvailableComPorts = useCallback(() => {
        if (broadcastOver === 'com') {
            console.log('getAvailableComPorts');
            invoke('cmd_get_available_com_ports', {
                requestId: uuidv4(),
            })
                .then((response) => {
                    const { data } = response as { data: string[] };
                    setAvailableComPorts(data.sort());
                    if (data.length > 0 && !data.includes(comPort)) {
                        setComPort(data[0]);
                    }
                })
                .catch((error) => {
                    console.error('Start', error);
                });
        }
    }, [broadcastOver, comPort, setComPort]);

    useEffect(() => {
        getAvailableComPorts();
    }, [getAvailableComPorts]);

    const broadcastOverOnChange = useCallback(
        (event: React.ChangeEvent<HTMLInputElement>) => {
            setBroadcastOver(event.target.value);
        },
        [setBroadcastOver],
    );

    const udpPortOnChange = useCallback(
        (event: React.ChangeEvent<HTMLInputElement>) => {
            setUdpPort(parseInt(event.target.value, 10));
        },
        [setUdpPort],
    );

    const udpNetmaskOnChange = useCallback(
        (event: React.ChangeEvent<HTMLInputElement>) => {
            setUdpNetmask(event.target.value);
        },
        [setUdpNetmask],
    );

    const comPortOnChange = useCallback(
        (event: SelectChangeEvent<string>) => {
            setComPort(event.target.value);
        },
        [setComPort],
    );

    const comBaudRateOnChange = useCallback(
        (event: React.ChangeEvent<HTMLInputElement>) => {
            setComBaudRate(parseInt(event.target.value, 10));
        },
        [setComBaudRate],
    );

    const broadcastDetails = useMemo(() => {
        if (broadcastOver === 'udp') {
            return (
                <Stack spacing={2} direction="row">
                    <FormControl component="fieldset">
                        <FormLabel component="legend">Broadcast address</FormLabel>
                        <Stack spacing={1} direction="row">
                            <Input
                                disabled={isDisabled}
                                name="udpNetmask"
                                id="udpNetmask"
                                inputComponent={IPAddressTextMask}
                                value={udpNetmask}
                                onChange={udpNetmaskOnChange}
                            />
                            <Tooltip
                                title={
                                    <Stack spacing={1} direction="column">
                                        <Typography variant="h5" component="h2">
                                            Use either
                                        </Typography>
                                        <Typography>
                                            &#x2022; a netmask like <strong>255.255.255.255</strong> if you want to
                                            broadcast to your entire network
                                        </Typography>
                                        <Typography>
                                            &#x2022; the exact IP address of the device you plan to use for navigation
                                        </Typography>
                                    </Stack>
                                }
                            >
                                <Help color={isDisabled ? 'disabled' : 'primary'} />
                            </Tooltip>
                        </Stack>
                    </FormControl>
                    <FormControl component="fieldset">
                        <FormLabel component="legend">Broadcast port</FormLabel>
                        <TextField
                            disabled={isDisabled}
                            name="udpPort"
                            id="udpPort"
                            type="number"
                            inputProps={{
                                min: 0,
                                max: 65536,
                            }}
                            InputLabelProps={{
                                shrink: true,
                            }}
                            value={udpPort}
                            onChange={udpPortOnChange}
                            variant="standard"
                            sx={{ minWidth: 120 }}
                        />
                    </FormControl>
                </Stack>
            );
        } else {
            return (
                <Stack spacing={2} direction="row">
                    <FormControl component="fieldset">
                        <FormLabel component="legend">Port</FormLabel>
                        <Stack spacing={1} direction="row">
                            <Select
                                disabled={isDisabled}
                                value={comPort}
                                onChange={comPortOnChange}
                                variant="standard"
                                sx={{ minWidth: 182 }}
                            >
                                {availableComPorts.map((port) => {
                                    return <MenuItem value={port}>{port}</MenuItem>;
                                })}
                            </Select>
                            <Tooltip
                                title={
                                    <Stack spacing={1} direction="column">
                                        <Typography variant="h5" component="h2">
                                            External tool required
                                        </Typography>
                                        <Typography>
                                            Please read the{' '}
                                            <a
                                                href="https://github.com/mihai-dinculescu/msfs-2020-gps-link/blob/add-com-support/instructions/COM.md"
                                                target="_blank"
                                                rel="noreferrer"
                                            >
                                                documentation
                                            </a>{' '}
                                            on how to set it up.
                                        </Typography>
                                    </Stack>
                                }
                            >
                                <Help color={isDisabled ? 'disabled' : 'primary'} />
                            </Tooltip>
                        </Stack>
                    </FormControl>
                    <FormControl component="fieldset">
                        <FormLabel component="legend">Baud rate</FormLabel>
                        <TextField
                            disabled={isDisabled}
                            name="comBaudRate"
                            id="comBaudRate"
                            type="number"
                            inputProps={{
                                min: 110,
                                max: 115200,
                            }}
                            InputLabelProps={{
                                shrink: true,
                            }}
                            value={comBaudRate}
                            onChange={comBaudRateOnChange}
                            variant="standard"
                            sx={{ minWidth: 120 }}
                        />
                    </FormControl>
                </Stack>
            );
        }
    }, [
        broadcastOver,
        udpNetmask,
        udpNetmaskOnChange,
        udpPort,
        udpPortOnChange,
        availableComPorts,
        comPort,
        comPortOnChange,
        comBaudRate,
        comBaudRateOnChange,
        isDisabled,
    ]);

    return (
        <>
            <FormControl component="fieldset">
                <FormLabel component="legend">Broadcast over</FormLabel>
                <RadioGroup
                    aria-label="Broadcast over"
                    name="broadcastOver"
                    value={broadcastOver}
                    onChange={broadcastOverOnChange}
                >
                    <FormControlLabel value="udp" control={<Radio disabled={isDisabled} />} label="UDP (X-Plane)" />
                    <FormControlLabel value="com" control={<Radio disabled={isDisabled} />} label="COM (RS232 GPS)" />
                </RadioGroup>
            </FormControl>
            <Box my={2}>{broadcastDetails}</Box>
        </>
    );
};
