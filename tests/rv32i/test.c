
#include <stdint.h>

// Random nonsense is okay for now

static uint32_t arr32[16] = { 0 };
static uint16_t arr16[16] = { 0 };
static uint8_t   arr8[16] = { 0 };

void _start(void) 
{
	uint32_t tmp = 0;

	for (int i = 0; i < 16; i++ ) {
		if (arr32[i] == 0) {
			arr32[i] = (uint32_t)i;
		}
		for (int j = 0; j < 32; j++) {
			tmp = 0xa5a5a5a5;
			tmp |= (arr32[i] << j);
			tmp ^= (arr32[i] << j);
			tmp &= (arr32[i] << j);
		}
	}

	for (int i = 0; i < 16; i++ ) {
		if (arr16[i] == 0) {
			arr16[i] = (uint16_t)i;
		}
		for (int j = 0; j < 16; j++) {
			tmp = 0xa5a5a5a5;
			tmp |= (arr16[i] << j);
			tmp ^= (arr16[i] << j);
			tmp &= (arr16[i] << j);
		}
	}

	for (int i = 0; i < 16; i++ ) {
		if (arr8[i] == 0) {
			arr8[i] = (uint8_t)i;
		}
		for (int j = 0; j < 8; j++) {
			tmp = 0xa5a5a5a5;
			tmp |= (arr16[i] << j);
			tmp ^= (arr16[i] << j);
			tmp &= (arr16[i] << j);
		}
	}

	return;

}
