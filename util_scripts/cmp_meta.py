def gen_cmp(cmps):
    res = [
        f"""
pub(crate) fn eval_{cmp}(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {{
    let ord = self._order(left, right)?;
    Ok(self.heap.alloc(ObjectInner::Bool(ord.{cmp}())))
}}
"""
        for cmp in cmps
    ]
    return "\n".join(res)


def gen_arith(ars):
    res = [
        f"""
pub(crate) fn eval_{a}(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {{
        let lobj = self.eval(left)?;
        let robj = self.eval(right)?;

        let lval = self.heap.get_value(lobj);
        let rval = self.heap.get_value(robj);

        match (lval, rval) {{
            (Value::Int(i1), Value::Int(i2)) => {{
                //
                Ok(self.heap.alloc(ObjectInner::Int(i1 {op} i2)))
            }}
            (Value::Float(f1), Value::Float(f2)) => {{
                Ok(self.heap.alloc(ObjectInner::Float(f1 {op} f2)))
            }}
            (l, r) => Err(TwiError::IncompatibleBinopType {{
                left: l.to_string(),
                right: r.to_string(),
            }}),
        }}
    }}
"""
        for a, op in ars
    ]
    return "\n".join(res)


if __name__ == "__main__":
    out_path = "code_generated.txt"

    res = ""

    cmps = ["lt", "gt", "leq", "geq"]
    res += gen_cmp(cmps)

    ars = [("add", "+"), ("minus", "-"), ("mul", "*"), ("div", "/")]
    res += gen_arith(ars)

    with open(out_path, "w") as file:
        file.write(res)

    print(f"Code is located in {out_path}")
