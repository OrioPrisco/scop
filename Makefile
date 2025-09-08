NAME=scop

all: $(NAME)

bonus: all

$(NAME): $(CARGO_TARGET_DIR)/debug/$(NAME)
	cp $< $@

$(CARGO_TARGET_DIR)/debug/$(NAME):
	cargo build

clean:
	cargo clean

fclean: clean
	rm -f $(NAME)

re: fclean all

.PHONY: all clean fclean re bonus
.SUFFIXES:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules
