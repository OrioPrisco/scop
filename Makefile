NAME=scop
TARGET_DIR=target/


all: $(NAME)

bonus: all

$(NAME): $(TARGET_DIR)/debug/$(NAME)
	cp $< $@

$(TARGET_DIR)/debug/$(NAME): src/
	docker build -t rust_scop .
	docker run --rm -v "$(PWD)":/usr/src/myapp -w /usr/src/myapp rust_scop cargo build

clean:
	rm -rf $(TARGET_DIR)

fclean: clean
	rm -f $(NAME)

re: fclean all

.PHONY: all clean fclean re bonus
.SUFFIXES:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules
