// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_datavalues::prelude::*;
use common_exception::Result;
use common_functions::scalars::Function;
use common_functions::scalars::SubstringFunction;
use pretty_assertions::assert_eq;

#[test]
fn test_substring_function() -> Result<()> {
    struct Test {
        name: &'static str,
        display: &'static str,
        nullable: bool,
        arg_names: Vec<&'static str>,
        columns: Vec<DataColumn>,
        expect: DataColumn,
        error: &'static str,
        func: Box<dyn Function>,
    }

    let schema = DataSchemaRefExt::create(vec![
        DataField::new("a", DataType::String, false),
        DataField::new("b", DataType::Int64, false),
        DataField::new("c", DataType::UInt64, false),
    ]);

    let tests = vec![
        Test {
            name: "substring-abcde-passed",
            display: "SUBSTRING",
            nullable: false,
            arg_names: vec!["a", "b", "c"],
            columns: vec![
                Series::new(vec!["abcde"]).into(),
                Series::new(vec![2_i64]).into(),
                Series::new(vec![3_u64]).into(),
            ],
            func: SubstringFunction::try_create("substring")?,
            expect: Series::new(vec!["bcd"]).into(),
            error: "",
        },
        Test {
            name: "substring-abcde-passed",
            display: "SUBSTRING",
            nullable: false,
            arg_names: vec!["a", "b", "c"],
            columns: vec![
                Series::new(vec!["abcde"]).into(),
                Series::new(vec![1_i64]).into(),
                Series::new(vec![3_u64]).into(),
            ],
            func: SubstringFunction::try_create("substring")?,
            expect: Series::new(vec!["abc"]).into(),
            error: "",
        },
        Test {
            name: "substring-abcde-passed",
            display: "SUBSTRING",
            nullable: false,
            arg_names: vec!["a", "b"],
            columns: vec![
                Series::new(vec!["abcde"]).into(),
                Series::new(vec![2_i64]).into(),
            ],

            func: SubstringFunction::try_create("substring")?,
            expect: Series::new(vec!["bcde"]).into(),
            error: "",
        },
        Test {
            name: "substring-1234567890-passed",
            display: "SUBSTRING",
            nullable: false,
            arg_names: vec!["a", "b", "c"],
            columns: vec![
                Series::new(vec!["1234567890"]).into(),
                Series::new(vec![-3_i64]).into(),
                Series::new(vec![3_u64]).into(),
            ],

            func: SubstringFunction::try_create("substring")?,
            expect: Series::new(vec!["890"]).into(),
            error: "",
        },
    ];

    for t in tests {
        let func = t.func;
        let rows = t.columns[0].len();

        // Type check.
        let mut args = vec![];
        let mut fields = vec![];
        for name in t.arg_names {
            args.push(schema.field_with_name(name)?.data_type().clone());
            fields.push(schema.field_with_name(name)?.clone());
        }

        let columns: Vec<DataColumnWithField> = t
            .columns
            .iter()
            .zip(fields.iter())
            .map(|(c, f)| DataColumnWithField::new(c.clone(), f.clone()))
            .collect();

        if let Err(e) = func.eval(&columns, rows) {
            assert_eq!(t.error, e.to_string(), "case: {}", t.name);
        }
        func.eval(&columns, rows)?;

        // Display check.
        let expect_display = t.display.to_string();
        let actual_display = format!("{}", func);
        assert_eq!(expect_display, actual_display, "case: {}", t.name);

        // Nullable check.
        let expect_null = t.nullable;
        let actual_null = func.nullable(&schema)?;
        assert_eq!(expect_null, actual_null, "case: {}", t.name);

        let v = &(func.eval(&columns, rows)?);

        // Type check.
        let expect_type = func.return_type(&args)?;
        let actual_type = v.data_type();
        assert_eq!(expect_type, actual_type, "case: {}", t.name);
        assert_eq!(v, &t.expect, "case: {}", t.name);
    }
    Ok(())
}