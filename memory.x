MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 255K
  PANICDUMP : ORIGIN = 0x2003FC00, LENGTH = 1K
}

_panic_dump_start = ORIGIN(PANICDUMP);
_panic_dump_end = ORIGIN(PANICDUMP) + LENGTH(PANICDUMP);