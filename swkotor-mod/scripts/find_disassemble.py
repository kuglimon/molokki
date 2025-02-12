import gdb

class FindAndDisassemble(gdb.Command):
    """Search for a byte sequence and disassemble matching addresses."""

    def __init__(self):
        super(FindAndDisassemble, self).__init__("find-disassemble", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        args = gdb.string_to_argv(arg)
        if len(args) < 3:
            print("Usage: find-disassemble <start_address> <end_address> <byte1> <byte2> ...")
            return

        print("starting")
        start_addr = int(args[0], 0)
        end_addr = int(args[1], 0)
        pattern = [int(byte, 0) for byte in args[2:]]
        print("okay")

        # Perform the search
        found_addr = gdb.execute(f"find /b {start_addr}, {end_addr}, " + ", ".join(hex(b) for b in pattern), to_string=True)
        if "pattern not found" in found_addr:
            print("No matches found.")
            return

        print(f"found matches")

        maybe_hit = []

        # Extract addresses and disassemble
        for line in found_addr.splitlines():
            if line.startswith("0x"):
                # address = line.split()[-1]
                address = line
                instruction = gdb.execute(f"x/1i {address}", to_string=True)

                if "320" in instruction:
                    maybe_hit.append(instruction)


        if len(maybe_hit) == 0:
            print("found nothing")
        else:
            print("found matches")
            for instruction in maybe_hit:
                print(instruction)


# Register the command in GDB
FindAndDisassemble()
