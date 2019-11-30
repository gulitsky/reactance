MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 1M
  RAM : ORIGIN = 0x20000000, LENGTH = 127K
  CCRAM: ORIGIN = 0x10000000, LENGTH = 64K
  PANICDUMP : ORIGIN = 0x2001FC00, LENGTH = 1K
}

_stack_start = ORIGIN(CCRAM) + LENGTH(CCRAM);

_panic_dump_start = ORIGIN(PANICDUMP);
_panic_dump_end = ORIGIN(PANICDUMP) + LENGTH(PANICDUMP);
