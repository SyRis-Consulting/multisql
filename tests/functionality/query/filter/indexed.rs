crate::util_macros::testcase!(
	(|mut glue: multisql::Glue| {
		assert!(
			matches!(
				glue.execute(r#"
					CREATE TABLE indexed (
						a INTEGER
					)
				"#),
				Ok(_)
			)
		);

		assert!(
			matches!(
				glue.execute(r#"
					INSERT INTO indexed (
						a
					) VALUES (
						1
					), (
						2
					), (
						3
					), (
						3
					), (
						4
					), (
						100
					)

				"#),
				Ok(_)
			)
		);

		crate::util_macros::assert_select!(glue, r#"
			SELECT
				a
			FROM
				indexed
		"# => a = I64: (1),(2),(3),(3),(4),(100));
		crate::util_macros::assert_select!(glue, r#"
			SELECT
				a
			FROM
				indexed
			WHERE
				a > 2
		"# => a = I64: (3),(3),(4),(100));
		crate::util_macros::assert_select!(glue, r#"
			SELECT
				a
			FROM
				indexed
			WHERE
				a < 4
		"# => a = I64: (1),(2),(3),(3));

		assert!(
			matches!(
				glue.execute(r#"
					CREATE INDEX index ON indexed (a)
				"#),
				Ok(_)
			)
		);

		crate::util_macros::assert_select!(glue, r#"
			SELECT
				a
			FROM
				indexed
		"# => a = I64: (1),(2),(3),(3),(4),(100));
		crate::util_macros::assert_select!(glue, r#"
			SELECT
				a
			FROM
				indexed
			WHERE
				a >= 3
		"# => a = I64: (3),(3),(4),(100));
		crate::util_macros::assert_select!(glue, r#"
			SELECT
				a
			FROM
				indexed
			WHERE
				a > 2
		"# => a = I64: (3),(3),(4),(100));
		crate::util_macros::assert_select!(glue, r#"
			SELECT
				a
			FROM
				indexed
			WHERE
				a <= 3
		"# => a = I64: (1),(2),(3),(3));
		crate::util_macros::assert_select!(glue, r#"
			SELECT
				a
			FROM
				indexed
			WHERE
				a < 4
		"# => a = I64: (1),(2),(3),(3));
	})
);
