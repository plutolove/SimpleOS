/*
#include <stdint.h>

struct idt_record
{
    uint16_t  limit;      /* Size of IDT array - 1 */
    uintptr_t base;       /* Pointer to IDT array  */
} __attribute__((packed));

void load_idt (struct idt_record *idt_r)
{
    __asm__ ("lidt %0" :: "m"(*idt_r));
}
*/
