import { FormControl, FormControlLabel, FormLabel, Radio, RadioGroup } from '@mui/material';
import { useCallback, useContext } from 'react';

import { ContactFormContext } from './ConnectForm';

export const ConnectFormRefreshRate: React.FC = () => {
    const { refreshRate, setRefreshRate, isDisabled } = useContext(ContactFormContext);

    const refreshRateOnChange = useCallback(
        (event: React.ChangeEvent<HTMLInputElement>) => {
            setRefreshRate(event.target.value);
        },
        [setRefreshRate],
    );

    return (
        <FormControl component="fieldset">
            <FormLabel component="legend">Refresh rate</FormLabel>
            <RadioGroup aria-label="Refresh Rate" name="refreshRate" value={refreshRate} onChange={refreshRateOnChange}>
                <FormControlLabel
                    value="fast"
                    control={<Radio disabled={isDisabled} />}
                    label="Fast (~ten times a second) "
                />
                <FormControlLabel value="slow" control={<Radio disabled={isDisabled} />} label="Slow (once a second)" />
            </RadioGroup>
        </FormControl>
    );
};
