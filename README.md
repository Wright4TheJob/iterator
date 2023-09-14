# Iterator

This is a small binary to count up over an arbitrary character set. This can be used for generating ID numbers with a large number of base characters tailored to your needs. This generally includes standard numerical IDs, Hex codes, full alphanumeric, or any other combination. 

The valid characterset is hard-coded currently. The intent is to have options to generate a number of next IDs or only the single next ID given the last ID. Sorting and ordering is based on the native string manipulation and ordering properties.

## Usage
The command line program takes 1 or 2 arguments. 
1. The last (or largest) ID used (required).
2. The number of next IDs to generate (Optional, default=1)

Example:
```
> iterator ABCD 3

ABCE
ABCF
ABCG
```