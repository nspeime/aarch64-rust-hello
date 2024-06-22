ENTRY(_start)

MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 128K
  RAM : ORIGIN = 0x40000000, LENGTH = 128K
}

SECTIONS
{
  .start : {
    *(._start)
  } > RAM AT> FLASH

  .text : {
    *(.text*)
  } > FLASH

  .data : {
    *(.data*)
  } > RAM

  .bss : {
    *(.bss*)
  } > RAM
}
