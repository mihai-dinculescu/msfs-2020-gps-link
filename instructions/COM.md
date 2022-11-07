# COM Setup

A pair of virtual COM ports must be created using an external tool. The paired ports will exchange serial data so that everything written to one port immediately appears in the other and vice versa.

## Example

If the pair created is **COM3 & COM4**, you can instruct MSFS 2020 GPS Link to broadcast to **COM3** and your navigation app to listen to **COM3**. The direction is irrelevant. You can alternatively have MSFS 2020 GPS Link broadcast to **COM4** and your navigation app listen to **COM3**. The important thing is to have MSFS 2020 GPS Link broadcast to one port, and your navigation app listen to the other.

## Tools that can create a pair of virtual COM ports

### [Null-modem emulator (com0com)](https://sourceforge.net/projects/com0com/)\

This setup will persist across multiple sessions and reboots, so you only need to do it once.

1. On the **Choose Components** page, check the **COM# <-> COM#** component.
2. On the **Completing the Null-modem Emulator Driver Setup Wizard** page, check **Launch Setup** and then click **Finish**.
3. Depending on the presence of .NET Framework 2.0 on your PC, you will either get a console window or a UI next.
    * if you get the console window, type

        ```bash
        change CNCA0 EmuOverrun=yes
        change CNCB0 EmuOverrun=yes
        ```

    * if you get the UI, check **enable buffer overrun** next to each port in the pair and then click **Apply**.
