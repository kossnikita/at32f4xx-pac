#[doc = "Register `OADDR2` reader"]
pub type R = crate::R<OADDR2_SPEC>;
#[doc = "Register `OADDR2` writer"]
pub type W = crate::W<OADDR2_SPEC>;
#[doc = "Field `ADDR2` reader - Own address 2"]
pub type ADDR2_R = crate::FieldReader;
#[doc = "Field `ADDR2` writer - Own address 2"]
pub type ADDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADDR2MASK` reader - Own address 2-bit mask"]
pub type ADDR2MASK_R = crate::FieldReader;
#[doc = "Field `ADDR2MASK` writer - Own address 2-bit mask"]
pub type ADDR2MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADDR2EN` reader - Own address 2 enable"]
pub type ADDR2EN_R = crate::BitReader;
#[doc = "Field `ADDR2EN` writer - Own address 2 enable"]
pub type ADDR2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:7 - Own address 2"]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Own address 2-bit mask"]
    #[inline(always)]
    pub fn addr2mask(&self) -> ADDR2MASK_R {
        ADDR2MASK_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Own address 2 enable"]
    #[inline(always)]
    pub fn addr2en(&self) -> ADDR2EN_R {
        ADDR2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OADDR2")
            .field("addr2", &format_args!("{}", self.addr2().bits()))
            .field("addr2mask", &format_args!("{}", self.addr2mask().bits()))
            .field("addr2en", &format_args!("{}", self.addr2en().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OADDR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 1:7 - Own address 2"]
    #[inline(always)]
    #[must_use]
    pub fn addr2(&mut self) -> ADDR2_W<OADDR2_SPEC> {
        ADDR2_W::new(self, 1)
    }
    #[doc = "Bits 8:10 - Own address 2-bit mask"]
    #[inline(always)]
    #[must_use]
    pub fn addr2mask(&mut self) -> ADDR2MASK_W<OADDR2_SPEC> {
        ADDR2MASK_W::new(self, 8)
    }
    #[doc = "Bit 15 - Own address 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn addr2en(&mut self) -> ADDR2EN_W<OADDR2_SPEC> {
        ADDR2EN_W::new(self, 15)
    }
}
#[doc = "Own address register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OADDR2_SPEC;
impl crate::RegisterSpec for OADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oaddr2::R`](R) reader structure"]
impl crate::Readable for OADDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oaddr2::W`](W) writer structure"]
impl crate::Writable for OADDR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OADDR2 to value 0"]
impl crate::Resettable for OADDR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
