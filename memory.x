MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* These correspond to NRF52840 with Softdevices S140 6.1.1 */
  FLASH : ORIGIN = 0x00026000, LENGTH = 868K
  /* CONFIG: ORIGIN = ORIGIN(FLASH) + LENGTH(FLASH), LENGTH = 0K */
  RAM : ORIGIN = 0x20020000, LENGTH = 128K
}

/*__config_start = ORIGIN(CONFIG) - ORIGIN(FLASH);*/
/*__config_end = __config_start + LENGTH(CONFIG); */
