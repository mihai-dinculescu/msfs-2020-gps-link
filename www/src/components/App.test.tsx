import { render, screen } from '@testing-library/react';
import { App } from './App';

test('renders the title', () => {
    render(<App />);
    const linkElement = screen.getByText(/MSFS 2020 GPS Link/i);
    expect(linkElement).toBeInTheDocument();
});
