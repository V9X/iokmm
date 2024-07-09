#include "iocom.h"
#include "mem.h"

NTSTATUS complete_request(PIRP irp, NTSTATUS status, ULONG_PTR inf) {
	irp->IoStatus.Status = status;
	irp->IoStatus.Information = inf;

	IoCompleteRequest(irp, IO_NO_INCREMENT);

	return status;
}

NTSTATUS io_create_close(PDEVICE_OBJECT dev_obj, PIRP irp) {
	UNREFERENCED_PARAMETER(dev_obj);

	return complete_request(irp, STATUS_SUCCESS, 0);
}

NTSTATUS io_control(PDEVICE_OBJECT dev_obj, PIRP irp) {
	UNREFERENCED_PARAMETER(dev_obj);

	PIO_STACK_LOCATION stack = IoGetCurrentIrpStackLocation(irp);

	// Return early if the size of the buffer does not match IoMessage size
	if (stack->Parameters.DeviceIoControl.InputBufferLength != sizeof(IoMessage)) {
		return complete_request(irp, STATUS_UNSUCCESSFUL, 0);
	}

	PIoMessage input = (PIoMessage)irp->AssociatedIrp.SystemBuffer;
	NTSTATUS status = STATUS_SUCCESS;
	SIZE_T return_size = 0;
	PEPROCESS process;

	if (!NT_SUCCESS(PsLookupProcessByProcessId(input->pid, &process))) {
		return complete_request(irp, STATUS_UNSUCCESSFUL, 0);
	}
	
	ULONG code = stack->Parameters.DeviceIoControl.IoControlCode;

	switch (code) {
		case IO_READ: {
			MmCopyVirtualMemory(process, input->address, PsGetCurrentProcess(), input->buffer, input->size, KernelMode, &return_size);
			break;
		};
		case IO_WRITE: {
			MmCopyVirtualMemory(PsGetCurrentProcess(), input->buffer, process, input->address, input->size, KernelMode, &return_size);
			break;
		}
		default: {
			status = STATUS_UNSUCCESSFUL;
		}
	}
	
	*(SIZE_T*)irp->AssociatedIrp.SystemBuffer = return_size;
	
	return complete_request(irp, status, sizeof(SIZE_T));
}
