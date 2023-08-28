#[doc = "Register `IDT` reader"]
pub type R = crate::R<IDT_SPEC>;
#[doc = "Field `IDT0` reader - Port input data"]
pub type IDT0_R = crate::BitReader;
#[doc = "Field `IDT1` reader - Port input data"]
pub type IDT1_R = crate::BitReader;
#[doc = "Field `IDT2` reader - Port input data"]
pub type IDT2_R = crate::BitReader;
#[doc = "Field `IDT3` reader - Port input data"]
pub type IDT3_R = crate::BitReader;
#[doc = "Field `IDT4` reader - Port input data"]
pub type IDT4_R = crate::BitReader;
#[doc = "Field `IDT5` reader - Port input data"]
pub type IDT5_R = crate::BitReader;
#[doc = "Field `IDT6` reader - Port input data"]
pub type IDT6_R = crate::BitReader;
#[doc = "Field `IDT7` reader - Port input data"]
pub type IDT7_R = crate::BitReader;
#[doc = "Field `IDT8` reader - Port input data"]
pub type IDT8_R = crate::BitReader;
#[doc = "Field `IDT9` reader - Port input data"]
pub type IDT9_R = crate::BitReader;
#[doc = "Field `IDT10` reader - Port input data"]
pub type IDT10_R = crate::BitReader;
#[doc = "Field `IDT11` reader - Port input data"]
pub type IDT11_R = crate::BitReader;
#[doc = "Field `IDT12` reader - Port input data"]
pub type IDT12_R = crate::BitReader;
#[doc = "Field `IDT13` reader - Port input data"]
pub type IDT13_R = crate::BitReader;
#[doc = "Field `IDT14` reader - Port input data"]
pub type IDT14_R = crate::BitReader;
#[doc = "Field `IDT15` reader - Port input data"]
pub type IDT15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Port input data"]
    #[inline(always)]
    pub fn idt0(&self) -> IDT0_R {
        IDT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input data"]
    #[inline(always)]
    pub fn idt1(&self) -> IDT1_R {
        IDT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input data"]
    #[inline(always)]
    pub fn idt2(&self) -> IDT2_R {
        IDT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input data"]
    #[inline(always)]
    pub fn idt3(&self) -> IDT3_R {
        IDT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input data"]
    #[inline(always)]
    pub fn idt4(&self) -> IDT4_R {
        IDT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input data"]
    #[inline(always)]
    pub fn idt5(&self) -> IDT5_R {
        IDT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input data"]
    #[inline(always)]
    pub fn idt6(&self) -> IDT6_R {
        IDT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input data"]
    #[inline(always)]
    pub fn idt7(&self) -> IDT7_R {
        IDT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input data"]
    #[inline(always)]
    pub fn idt8(&self) -> IDT8_R {
        IDT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port input data"]
    #[inline(always)]
    pub fn idt9(&self) -> IDT9_R {
        IDT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port input data"]
    #[inline(always)]
    pub fn idt10(&self) -> IDT10_R {
        IDT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port input data"]
    #[inline(always)]
    pub fn idt11(&self) -> IDT11_R {
        IDT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port input data"]
    #[inline(always)]
    pub fn idt12(&self) -> IDT12_R {
        IDT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port input data"]
    #[inline(always)]
    pub fn idt13(&self) -> IDT13_R {
        IDT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port input data"]
    #[inline(always)]
    pub fn idt14(&self) -> IDT14_R {
        IDT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port input data"]
    #[inline(always)]
    pub fn idt15(&self) -> IDT15_R {
        IDT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "GPIO input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
