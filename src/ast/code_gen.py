from typing import Iterable


def smart_num_int_convert() -> Iterable[str]:
    len_list = (8, 16, 32, 64)
    sign_list = ("u", "i")
    for l in len_list:
        for s in sign_list:
            tp = f"{s}{l}"
            code = (f"impl From<{tp}> for SmartNum{{\n" +
                    f"    fn from(v: {tp}) -> Self {{\n" +
                    "        SmartNum::Integer(i64::from(v))\n" +
                    "    }\n" +
                    "}\n")
            yield code


def operand_int_convert() -> Iterable[str]:
    len_list = (8, 16, 32, 64)
    sign_list = ("u", "i")
    tp_list = ["f32", "f64"]
    for l in len_list:
        for s in sign_list:
            tp = f"{s}{l}"
            tp_list.append(tp)
    for tp in tp_list:
        code = (f"impl From<{tp}> for AstOperand{{\n" +
                f"    fn from(v: {tp}) -> Self {{\n" +
                "        AstOperand::Num(SmartNum::from(v))\n" +
                "    }\n" +
                "}\n")
        yield code


def node_convert() -> Iterable[str]:
    len_list = (8, 16, 32, 64)
    sign_list = ("u", "i")
    tp_list = ["f32", "f64"]
    for l in len_list:
        for s in sign_list:
            tp = f"{s}{l}"
            tp_list.append(tp)
    for tp in tp_list:
        code = (f"impl From<{tp}> for AstNode {{\n" +
                f"    fn from(v: {tp}) -> Self {{\n" +
                "         AstNode {\n" +
                "             me: AstNodeTag::Operand(AstOperand::from(v)),\n" +
                "             child: vec![],\n" +
                "         }\n" +
                "    }\n" +
                "}\n")
        yield code


if __name__ == "__main__":
    for code in smart_num_int_convert():
        print(code)
    print("\n\n")
    for code in operand_int_convert():
        print(code)
    print("\n\n")
    for code in node_convert():
        print(code)
