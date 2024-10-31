def eq_gen(funcs):
    def chunk(variant, func):
        return f"\nif let Token::{variant} = self.current() {{\n    eq_continue = true;\n    self.cur += 1;\n    let right = self.parse_add()?;\n    left = ExprNode::{func}(left, right);\n    continue;\n}}\n"

    return "\n".join(chunk(variant, func) for variant, func in funcs)


def comparison_gen(funcs):
    def chunk(variant, func):
        return f"\nif let Token::{variant} = self.current() {{\n    cmp_continue = true;\n    self.cur += 1;\n    let right = self.parse_add()?;\n    left = ExprNode::{func}(left, right);\n    continue;\n}}\n"

    return "\n".join(chunk(variant, func) for variant, func in funcs)


def add_gen(funcs):
    def chunk(variant, func):
        return f"\nif let Token::{variant} = self.current() {{\n    add_continue = true;\n    self.cur += 1;\n    let right = self.parse_add()?;\n    left = ExprNode::{func}(left, right);\n    continue;\n}}\n"

    return "\n".join(chunk(variant, func) for variant, func in funcs)


def mul_gen(funcs):
    def chunk(variant, func):
        return f"\nif let Token::{variant} = self.current() {{\n    mul_continue = true;\n    self.cur += 1;\n    let right = self.parse_add()?;\n    left = ExprNode::{func}(left, right);\n    continue;\n}}\n"

    return "\n".join(chunk(variant, func) for variant, func in funcs)


if __name__ == "__main__":
    out_path = "code_generated.txt"
    res = ""

    binops = [("Eq", "eq"), ("Neq", "neq")]
    res += eq_gen(binops)
    print(res)

    binops = [("Gt", "gt"), ("Geq", "geq"), ("Lt", "lt"), ("Leq", "leq")]
    res += comparison_gen(binops)
    print(res)

    binops = [("Plus", "add"), ("Minus", "minus")]
    res += add_gen(binops)
    print(res)

    binops = [("Star", "mul"), ("Slash", "div")]
    res += mul_gen(binops)
    print(res)

    # 将生成的代码写入文件
    with open(out_path, "w") as file:
        file.write(res)

    print(f"Code is located in {out_path}")
