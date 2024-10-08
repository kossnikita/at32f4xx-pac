#[doc = "Register `CTRLSTS2` reader"]
pub type R = crate::R<CTRLSTS2_SPEC>;
#[doc = "Register `CTRLSTS2` writer"]
pub type W = crate::W<CTRLSTS2_SPEC>;
#[doc = "Field `COMP1NINVSEL` reader - Comparator1 non-inverting input selection"]
pub type COMP1NINVSEL_R = crate::FieldReader;
#[doc = "Field `COMP1NINVSEL` writer - Comparator1 non-inverting input selection"]
pub type COMP1NINVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2NINVSEL` reader - Comparator2 non-inverting input selection"]
pub type COMP2NINVSEL_R = crate::FieldReader;
#[doc = "Field `COMP2NINVSEL` writer - Comparator2 non-inverting input selection"]
pub type COMP2NINVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Comparator1 non-inverting input selection"]
    #[inline(always)]
    pub fn comp1ninvsel(&self) -> COMP1NINVSEL_R {
        COMP1NINVSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - Comparator2 non-inverting input selection"]
    #[inline(always)]
    pub fn comp2ninvsel(&self) -> COMP2NINVSEL_R {
        COMP2NINVSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS2")
            .field("comp1ninvsel", &self.comp1ninvsel())
            .field("comp2ninvsel", &self.comp2ninvsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator1 non-inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1ninvsel(&mut self) -> COMP1NINVSEL_W<CTRLSTS2_SPEC> {
        COMP1NINVSEL_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Comparator2 non-inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2ninvsel(&mut self) -> COMP2NINVSEL_W<CTRLSTS2_SPEC> {
        COMP2NINVSEL_W::new(self, 16)
    }
}
#[doc = "CMP control/status register2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLSTS2_SPEC;
impl crate::RegisterSpec for CTRLSTS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts2::R`](R) reader structure"]
impl crate::Readable for CTRLSTS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts2::W`](W) writer structure"]
impl crate::Writable for CTRLSTS2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLSTS2 to value 0"]
impl crate::Resettable for CTRLSTS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
