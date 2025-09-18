#include <stdint.h>
#include <stdio.h>

/////////////////////// ANCHOR: extern_struct
struct Data
{
	uint32_t a;
	uint16_t b;
	uint64_t c;
};
__attribute__((packed)) struct PackedData
{
	uint32_t a;
	uint16_t b;
	uint64_t c;
};
/////////////////////// ANCHOR_END: extern_struct

/////////////////////// ANCHOR: pointers
#include <stdint.h>
#include <inttypes.h>

//! Add in place
void add_in_place(uint32_t *a, uint32_t b);

int use_add_in_place()
{
	uint32_t x = 25;
	add_in_place(&x, 17);
	printf("%" PRIu32 " == 42", x);
	return 0;
}
/////////////////////// ANCHOR_END: pointers

/////////////////////// ANCHOR: function_pointers
uint32_t repeat(uint32_t start, uint32_t n, uint32_t (*f)(uint32_t));
/////////////////////// ANCHOR_END: function_pointers

/////////////////////// ANCHOR: free_intern
struct XtraResource;
void xtra_with(void (*cb)(struct XtraResource *xtra));
void xtra_sthg(struct XtraResource *xtra);

void cb(struct XtraResource *xtra)
{
	// ()...) do anything with the proposed C API for XtraResource
	xtra_sthg(xtra);
}

int use_xtra()
{
	xtra_with(cb);
}
/////////////////////// ANCHOR_END: free_intern
