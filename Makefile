# ========================
# 基本工具
# ========================

CODEQL ?= codeql
LANG   := cpp

# ========================
# 源码路径（按需改）
# ========================

LINUX_SRC ?= /path/to/linux
RMM_SRC   ?= /path/to/rmm
KATA_SRC  ?= /path/to/kata

# ========================
# 数据库 & 输出
# ========================

DB_ROOT  := db
OUT_ROOT := out

LINUX_DB := $(DB_ROOT)/linux
RMM_DB   := $(DB_ROOT)/rmm
KATA_DB  := $(DB_ROOT)/kata

# ========================
# 查询路径
# ========================

LINUX_QUERIES := $(shell find query/linux -name "*.ql")
KATA_QUERIES  := $(shell find query/kata  -name "*.ql")

# RMM 通常和 Linux 共用 C/C++ 查询
RMM_QUERIES   := $(LINUX_QUERIES)

# ========================
# 默认目标
# ========================

.PHONY: all
all: linux rmm kata

# ========================
# Linux
# ========================

.PHONY: linux
linux: linux-db linux-run linux-csv

linux-db:
	@echo "[+] Creating Linux CodeQL database"
	$(CODEQL) database create $(LINUX_DB) \
		--language=$(LANG) \
		--source-root=$(LINUX_SRC) \
		--command="make defconfig && make -j$$(nproc)" \
		--overwrite

linux-run:
	@mkdir -p $(OUT_ROOT)/linux
	@for q in $(LINUX_QUERIES); do \
		name=$$(basename $$q .ql); \
		echo "[linux] $$name"; \
		$(CODEQL) query run $$q \
			--database=$(LINUX_DB) \
			--output=$(OUT_ROOT)/linux/$$name.bqrs; \
	done

linux-csv:
	@for f in $(OUT_ROOT)/linux/*.bqrs; do \
		name=$$(basename $$f .bqrs); \
		$(CODEQL) bqrs decode $$f \
			--format=csv \
			--output=$(OUT_ROOT)/linux/$$name.csv; \
	done

# ========================
# RMM
# ========================

.PHONY: rmm
rmm: rmm-db rmm-run rmm-csv

rmm-db:
	@echo "[+] Creating RMM CodeQL database"
	$(CODEQL) database create $(RMM_DB) \
		--language=$(LANG) \
		--source-root=$(RMM_SRC) \
		--command="make -j$$(nproc)" \
		--overwrite

rmm-run:
	@mkdir -p $(OUT_ROOT)/rmm
	@for q in $(RMM_QUERIES); do \
		name=$$(basename $$q .ql); \
		echo "[rmm] $$name"; \
		$(CODEQL) query run $$q \
			--database=$(RMM_DB) \
			--output=$(OUT_ROOT)/rmm/$$name.bqrs; \
	done

rmm-csv:
	@for f in $(OUT_ROOT)/rmm/*.bqrs; do \
		name=$$(basename $$f .bqrs); \
		$(CODEQL) bqrs decode $$f \
			--format=csv \
			--output=$(OUT_ROOT)/rmm/$$name.csv; \
	done

# ========================
# Kata
# ========================

.PHONY: kata
kata: kata-db kata-run kata-csv

kata-db:
	@echo "[+] Creating Kata CodeQL database"
	$(CODEQL) database create $(KATA_DB) \
		--language=$(LANG) \
		--source-root=$(KATA_SRC) \
		--command="make -j$$(nproc)" \
		--overwrite

kata-run:
	@mkdir -p $(OUT_ROOT)/kata
	@for q in $(KATA_QUERIES); do \
		name=$$(basename $$q .ql); \
		echo "[kata] $$name"; \
		$(CODEQL) query run $$q \
			--database=$(KATA_DB) \
			--output=$(OUT_ROOT)/kata/$$name.bqrs; \
	done

kata-csv:
	@for f in $(OUT_ROOT)/kata/*.bqrs; do \
		name=$$(basename $$f .bqrs); \
		$(CODEQL) bqrs decode $$f \
			--format=csv \
			--output=$(OUT_ROOT)/kata/$$name.csv; \
	done

# ========================
# 清理
# ========================

.PHONY: clean
clean:
	rm -rf $(DB_ROOT) $(OUT_ROOT)
