const SELECT: string = "select";
const CREATE: string = "create";
const INSERT: string = "insert";
const UPDATE: string = "update";
const DELETE: string = "delete";
const DROP: string = "drop";
const FROM: string = "from";
const INTO: string = "into";
const WHERE: string = "where";
const DISTINCT: string = "distinct";
const ALTER: string = "alter";
const ADD: string = "add";
const SET: string = "set";
const TRUNCATE: string = "truncate";
const AS: string = "as";
const ASC: string = "asc";
const DESC: string = "desc";
const BETWEEN: string = "between";
const HAVING: string = "having";
const IN: string = "in";
const JOIN: string = "join";
const EXISTS: string = "exists";
const LIKE: string = "like";
const CASE: string = "case";

pub fn get_keywords() -> Vec<string> {
    return Vec![
        SELECT, CREATE, INSERT, UPDATE, DELETE, DROP, FROM, INTO, WHERE, DISTINCT, ALTER, ADD, SET,
        ASC, DESC, BETWEEN, HAVING, IN, JOIN, EXISTS, LIKE, CASE
    ];
}
