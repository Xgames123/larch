# Other devices to communicate with


## chardev_ascii
| addr | description                                                    |
|------|----------------------------------------------------------------|
| 0xF0 | Char out low. The low nibble of the character to be outputted  |
| 0xF1 | Char out high. The high nibble of the caracter to be outputted |
| 0xF2 | When a character is available. The low nib is put here else 0  |
| 0xF2 | When a character is available. The high nib is put here else 0 |

