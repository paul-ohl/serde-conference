use std::collections::HashMap;

use serde::{ser, Serialize};

use pobl::error::{Error, Result};

pub struct Serializer {
    output: String,
    indent_level: usize,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
        indent_level: 0,
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output += if v { "✔️" } else { "❌" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }
    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.output.push(v);
        Ok(())
    }
    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += v;
        Ok(())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output += &String::from_utf8_lossy(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.output += "👻";
        Ok(())
    }
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.serialize_none()
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_none()
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        self.output += "_ ";
        name.serialize(&mut *self)?;
        self.output += &format!(" {{\n{}", "    ".repeat(self.indent_level + 1));
        variant.serialize(&mut *self)?;
        self.output += " => ";
        value.serialize(&mut *self)?;
        self.output += &format!("\n{}}}", "    ".repeat(self.indent_level));
        Ok(())
    }

    // Sequences
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output += "[";
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output += "_ ";
        name.serialize(&mut *self)?;
        self.output += &format!(" {{\n{}", "    ".repeat(self.indent_level + 1));
        variant.serialize(&mut *self)?;
        self.output += " => [";
        self.indent_level += 1;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if len.is_none() || len == Some(0) {
            self.output += "{";
        } else {
            self.output += &format!("{{\n{}", "    ".repeat(self.indent_level + 1));
        }
        self.indent_level += 1;
        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.output += "_ ";
        self.output += name;
        self.output += &format!(" {{\n{}", "    ".repeat(self.indent_level + 1));
        self.indent_level += 1;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.indent_level += 1;

        self.output += "_ ";
        self.output += name;
        self.output += &format!(" {{\n{}_ ", "    ".repeat(self.indent_level));
        variant.serialize(&mut *self)?;

        self.indent_level += 1;
        self.output += &format!(" {{\n{}", "    ".repeat(self.indent_level));
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += " - ";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.trim().ends_with('[') {
            self.output += " - ";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.trim().ends_with('[') {
            self.output += " - ";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.trim().ends_with('[') {
            self.output += " - ";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.indent_level -= 1;
        self.output += &format!("]\n{}}}", "    ".repeat(self.indent_level));
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.trim().ends_with('{') {
            self.output += &format!("\n{}", "    ".repeat(self.indent_level));
        }
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += " => ";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.indent_level -= 1;
        self.output += &format!("\n{}}}", "    ".repeat(self.indent_level));
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.trim().ends_with('{') {
            self.output += &format!("\n{}", "    ".repeat(self.indent_level));
        }
        key.serialize(&mut **self)?;
        self.output += " => ";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.indent_level -= 1;
        self.output += &format!("\n{}}}", "    ".repeat(self.indent_level));
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.trim().ends_with('{') {
            self.output += &format!("\n{}", "    ".repeat(self.indent_level));
        }
        key.serialize(&mut **self)?;
        self.output += " => ";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.indent_level -= 2;
        self.output += &format!(
            "\n{}}}\n{}}}",
            "    ".repeat(self.indent_level + 1),
            "    ".repeat(self.indent_level)
        );
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_struct() {
    #[derive(Serialize)]
    struct Test {
        number: u32,
        seq: Vec<&'static str>,
    }

    let test = Test {
        number: 1,
        seq: vec!["a", "b"],
    };
    let expected = r#"
_ Test {
    number => 1
    seq => [a - b]
}
    "#;
    assert_eq!(to_string(&test).unwrap(), expected.trim());
}

#[test]
fn test_enum() {
    #[derive(Serialize)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32, b: u32 },
    }

    #[derive(Serialize)]
    struct S {
        e: E,
        m: HashMap<&'static str, &'static str>,
    }

    let u = E::Unit;
    let expected = "Unit";
    assert_eq!(to_string(&u).unwrap(), expected);

    let n = E::Newtype(1);
    let expected = r#"
_ E {
    Newtype => 1
}
    "#;
    assert_eq!(to_string(&n).unwrap(), expected.trim());

    let t = E::Tuple(1, 2);
    let expected = r#"
_ E {
    Tuple => [1 - 2]
}
    "#;
    assert_eq!(to_string(&t).unwrap(), expected.trim());

    let s = E::Struct { a: 1, b: 2 };
    let expected = r#"
_ E {
    _ Struct {
        a => 1
        b => 2
    }
}
    "#;
    assert_eq!(to_string(&s).unwrap(), expected.trim());

    let mut m = HashMap::new();
    m.insert("a", "b");
    m.insert("c", "d");
    let mut s = S {
        e: E::Struct { a: 1, b: 2 },
        m,
    };
    let expected = r#"
_ S {
    e => _ E {
        _ Struct {
            a => 1
            b => 2
        }
    }
    m => {
        a => b
        c => d
    }
}
    "#;
    assert_eq!(to_string(&s).unwrap(), expected.trim());
}
