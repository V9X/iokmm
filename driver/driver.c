#include <ntifs.h>
#include "iocom.h"

#define NT_DEVICE_NAME L"\\Device\\IOkmm"
#define DOS_DEVICE_NAME L"\\DosDevices\\IOkmm"

void unload(PDRIVER_OBJECT drv_obj) {
	UNICODE_STRING dos_dev_name;
	PDEVICE_OBJECT dev_obj = drv_obj->DeviceObject;
	
	RtlInitUnicodeString(&dos_dev_name, DOS_DEVICE_NAME);
	IoDeleteSymbolicLink(&dos_dev_name);

	if (dev_obj != NULL) {
		IoDeleteDevice(dev_obj);
	}
}

NTSTATUS DriverEntry(PDRIVER_OBJECT drv_obj, PUNICODE_STRING reg_path) {
	UNREFERENCED_PARAMETER(reg_path);

	PDEVICE_OBJECT dev_obj;
	UNICODE_STRING nt_dev_name, dos_dev_name;

	RtlInitUnicodeString(&nt_dev_name, NT_DEVICE_NAME);
	NTSTATUS status = IoCreateDevice(drv_obj, 0, &nt_dev_name, FILE_DEVICE_UNKNOWN, FILE_DEVICE_SECURE_OPEN, FALSE, &dev_obj);
	if (!NT_SUCCESS(status)) return status;

	RtlInitUnicodeString(&dos_dev_name, DOS_DEVICE_NAME);
	status = IoCreateSymbolicLink(&dos_dev_name, &nt_dev_name);
	if (!NT_SUCCESS(status)) {
		IoDeleteDevice(dev_obj);
		return status;
	}

	drv_obj->MajorFunction[IRP_MJ_CREATE] = io_create_close;
	drv_obj->MajorFunction[IRP_MJ_CLOSE] = io_create_close;
	drv_obj->MajorFunction[IRP_MJ_DEVICE_CONTROL] = io_control;
	drv_obj->DriverUnload = unload;

	SetFlag(dev_obj->Flags, DO_BUFFERED_IO);
	ClearFlag(dev_obj->Flags, DO_DEVICE_INITIALIZING);

	return STATUS_SUCCESS;
}
