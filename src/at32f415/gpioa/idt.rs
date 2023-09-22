#[doc = "Register `IDT` reader"]
pub type R = crate::R<IDT_SPEC>;
#[doc = "Field `IDT[0-15]` reader - Port input data"]
pub type IDT_R = crate::BitReader<IDT0_A>;
#[doc = "Port input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDT0_A {
    #[doc = "0: Input is logic low"]
    Low = 0,
    #[doc = "1: Input is logic high"]
    High = 1,
}
impl From<IDT0_A> for bool {
    #[inline(always)]
    fn from(variant: IDT0_A) -> Self {
        variant as u8 != 0
    }
}
impl IDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDT0_A {
        match self.bits {
            false => IDT0_A::Low,
            true => IDT0_A::High,
        }
    }
    #[doc = "Input is logic low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDT0_A::Low
    }
    #[doc = "Input is logic high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDT0_A::High
    }
}
impl R {
    #[doc = "Port input data"]
    #[inline(always)]
    pub unsafe fn idt(&self, n: u8) -> IDT_R {
        IDT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Port input data"]
    #[inline(always)]
    pub fn idt0(&self) -> IDT_R {
        IDT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input data"]
    #[inline(always)]
    pub fn idt1(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input data"]
    #[inline(always)]
    pub fn idt2(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input data"]
    #[inline(always)]
    pub fn idt3(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input data"]
    #[inline(always)]
    pub fn idt4(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input data"]
    #[inline(always)]
    pub fn idt5(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input data"]
    #[inline(always)]
    pub fn idt6(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input data"]
    #[inline(always)]
    pub fn idt7(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input data"]
    #[inline(always)]
    pub fn idt8(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port input data"]
    #[inline(always)]
    pub fn idt9(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port input data"]
    #[inline(always)]
    pub fn idt10(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port input data"]
    #[inline(always)]
    pub fn idt11(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port input data"]
    #[inline(always)]
    pub fn idt12(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port input data"]
    #[inline(always)]
    pub fn idt13(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port input data"]
    #[inline(always)]
    pub fn idt14(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port input data"]
    #[inline(always)]
    pub fn idt15(&self) -> IDT_R {
        IDT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDT_SPEC;
impl crate::RegisterSpec for IDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idt::R`](R) reader structure"]
impl crate::Readable for IDT_SPEC {}
#[doc = "`reset()` method sets IDT to value 0"]
impl crate::Resettable for IDT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
