# COM Setup

A pair of virtual COM ports must be created using an external tool. The paired ports will exchange serial data so that everything written to one port immediately appears in the other and vice versa.

## Example

If the pair created is **COM3 & COM4**, you can instruct MSFS 2020 GPS Link to broadcast to **COM3** and your navigation app to listen to **COM3**. The direction is irrelevant. You can alternatively have MSFS 2020 GPS Link broadcast to **COM4** and your navigation app listen to **COM3**. The important thing is to have MSFS 2020 GPS Link broadcast to one port, and your navigation app listen to the other.

## Tools that can create a pair of virtual COM ports

- [Null-modem emulator (com0com)](https://sourceforge.net/projects/com0com/)\
When installing, make sure to select the **COM# <-> COM#** component. This will create one pair of virtual COM ports for you to use. There should be nothing else to do after the installation completes.
