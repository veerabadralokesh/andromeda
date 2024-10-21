func parseBoolExpr(expression string) bool {
    i := 0
    return parse(&expression, &i)
}

func parse(expr *string, i *int) bool {
    if (*expr)[*i] == 't' {
        *i++
        return true
    }
    if (*expr)[*i] == 'f' {
        *i++
        return false
    }
    if (*expr)[*i] == '!' {
        *i += 2
        ans := !parse(expr, i)
        *i++
        return ans
    }
    
    isAnd := false

    if (*expr)[*i] == '&' {
        isAnd = true
    }
    *i += 2
    
    ans := isAnd

    for (*expr)[*i] != ')' {
        parsedExpr := parse(expr, i)
        if isAnd {
            ans = ans && parsedExpr
        } else {
            ans = ans || parsedExpr
        }
        if (*expr)[*i] == ',' {
            *i++
        }
    }
    *i++
    return ans
}

