import { InputBaseComponentProps } from '@material-ui/core';
import MaskedInput from 'react-text-mask';

interface TextMaskCustomProps extends InputBaseComponentProps {
    inputRef?: (ref: HTMLElement | null) => void;
}

export const IPAddressTextMask: React.FC = (props: TextMaskCustomProps) => {
    const { inputRef, ...other } = props;

    return (
        <MaskedInput
            {...other}
            ref={(instance) => {
                if (inputRef) {
                    inputRef(instance ? instance.inputElement : null);
                }
            }}
            placeholderChar={'\u2000'}
            showMask
            guide={false}
            mask={(value: string) => {
                const mask = Array(value.length).fill(/[\d.]/);
                const chunks = value.split('.');

                for (let i = 0; i < 4; ++i) {
                    const chunk = chunks[i] || '';

                    if (255 % +chunk === 255) {
                        mask[value.length - 1] = '.';
                        mask[value.length] = /[\d.]/;
                    }
                }

                return mask;
            }}
            pipe={(value: string) => {
                if (value === '.' || value.endsWith('..')) return false;

                const parts = value.split('.');

                if (
                    parts.length > 4 ||
                    parts.some((part) => part === '00' || parseInt(part, 10) < 0 || parseInt(part) > 255)
                ) {
                    return false;
                }

                return value;
            }}
        />
    );
};
