# Other devices to communicate with


## chardev_ascii
| port addr | description                                                     |
|-----------|-----------------------------------------------------------------|
| 0         | Char out low. The low nibble of the character to be outputted   |
| 1         | Char out high. The high nibble of the character to be outputted |
| 2         | When a character is available. The low nib is put here else 0   |
| 3         | When a character is available. The high nib is put here else 0  |
