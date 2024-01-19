/*
    Might be usefull later
*/

struct SavedAnchor {
    anchor: super::Anchor,
    offset: super::Vector,
}

pub struct Rect {
    anchor: Option<SavedAnchor>,
    pos: super::Vector,
    size: super::Vector,
}

impl Rect {
    pub fn new(pos: impl Into<super::Vector>, size: impl Into<super::Vector>) -> Self {
        Self {
            pos: pos.into(),
            size: size.into(),
            anchor: None,
        }
    }
    pub fn new_anchored(
        anchor: impl Into<super::Anchor>,
        offset: impl Into<super::Vector>,
        size: impl Into<super::Vector>,
    ) -> Self {
        let size = size.into();
        let anchor = anchor.into();
        let offset = offset.into();
        let pos = anchor.as_value(size.clone()) + offset.clone();
        Self {
            pos,
            size,
            anchor: Some(SavedAnchor { anchor, offset }),
        }
    }

    pub fn set_size(&mut self, new_size: impl Into<super::Vector>) {
        self.size = new_size.into();
        if let Some(anchor) = &self.anchor {
            self.pos = anchor.anchor.as_value(self.size.clone()) + anchor.offset.clone();
        }
    }
    #[inline]
    pub fn position(&self) -> super::Vector {
        self.pos.clone()
    }

    #[inline]
    pub fn pos(&self) -> super::Vector {
        self.position()
    }

    #[inline]
    pub fn size(&self) -> super::Vector {
        self.size.clone()
    }

    #[inline]
    pub fn x(&self) -> super::Value{
        self.pos.x()
    }
    #[inline]
    pub fn y(&self) -> super::Value{
        self.pos.y()
    } 
    #[inline]
    pub fn w(&self) -> super::Value{
        self.size.x()
    }
    #[inline]
    pub fn h(&self) -> super::Value{
        self.size.y()
    }
}
