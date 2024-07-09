#pragma once
#include <ntifs.h>

#define IO_READ CTL_CODE(FILE_DEVICE_UNKNOWN, 0x1, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define IO_WRITE CTL_CODE(FILE_DEVICE_UNKNOWN, 0x2, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)

typedef struct _IoMessage {
	HANDLE pid; // Process id
	SIZE_T size; // Size of the buffer
	PVOID buffer; // Usermode process buffer
	PVOID address; // Address from which to read / overwrite
} IoMessage, * PIoMessage;

NTSTATUS io_create_close(PDEVICE_OBJECT dev_obj, PIRP irp);

NTSTATUS io_control(PDEVICE_OBJECT dev_obj, PIRP irp);