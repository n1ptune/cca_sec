for fuzz:
    replace normal world include linux,u-boot with rmm_fuzz
    ```
    BL33_BIN		?= $(RMM_FUZZ_BIN)

    TF_A_FLAGS ?= \
        BL33=$(BL33_BIN) \
        PLAT=qemu \
        QEMU_USE_GIC_DRIVER=$(TFA_GIC_DRIVER) \
        DEBUG=$(TF_A_DEBUG) \
        LOG_LEVEL=$(TF_A_LOGLVL) \
        ENABLE_RME=1 \
        RMM=$(RMM_BIN)

    AFL_PATH ?= $(ROOT)/AFL
    WORK_PATH ?= $(ROOT)/test
    .PHONY: fuzz
    fuzz:
        ln -rsf $(ROOT)/out-br/images/rootfs.cpio.gz $(BINARIES_PATH)/
        cd $(BINARIES_PATH) && afl-fuzz -i $(WORK_PATH)/in -o $(WORK_PATH)/out -m none -Q 1 $(QEMU_BUILD)/qemu-system-aarch64 \
            -M virt,virtualization=on,secure=on,gic-version=3 \
            -M acpi=off -cpu max,x-rme=on,sme=off,pauth-impdef=on \
            -m 3G \
            -nographic \
            -bios flash.bin \
            -drive format=qcow2,if=none,file=$(ROOT)/out-br/images/rootfs.qcow2,id=hd0 \
            -nodefaults \
            --accel tcg,thread=single \
            -loadvm booted \
            -aflFile @@

    .PHONY: dbg
    dbg:
        ln -rsf $(ROOT)/out-br/images/rootfs.cpio.gz $(BINARIES_PATH)/
        cd $(BINARIES_PATH) && $(QEMU_BUILD)/qemu-system-aarch64 \
            -M virt,virtualization=on,secure=on,gic-version=3 \
            -M acpi=off -cpu max,x-rme=on,sme=off,pauth-impdef=on \
            -m 3G \
            -nographic \
            -bios flash.bin \
            -kernel Image \
            -drive format=qcow2,if=none,file=$(ROOT)/out-br/images/rootfs.qcow2,id=hd0 \
            -device virtio-blk-pci,drive=hd0 \
            -append root=/dev/vda \
            -nodefaults \
            -append "root=/dev/vda earlycon console=hvc0 nokaslr" \
            -device virtio-net-pci,netdev=net0 \
            -netdev user,id=net0 \
            --accel tcg,thread=single \
            -monitor telnet::4444,server,nowait \
            -s -S
    ```
for query:
    install codeql env and create database for linux, rmm, kata and then run query;
    ```
    make linux-db
    make rmm-db
    make kata-db

    //It is recommended to run using the CodeQL extension for VS Code.
    make linux-run
    make rmm-run
    make kata-run
    ```
