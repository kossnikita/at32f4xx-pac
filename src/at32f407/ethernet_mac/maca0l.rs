#[doc = "Register `MACA0L` reader"]
pub type R = crate::R<MACA0L_SPEC>;
#[doc = "Register `MACA0L` writer"]
pub type W = crate::W<MACA0L_SPEC>;
#[doc = "Field `MA0L` reader - MAC address0 low"]
pub type MA0L_R = crate::FieldReader<u32>;
#[doc = "Field `MA0L` writer - MAC address0 low"]
pub type MA0L_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address0 low"]
    #[inline(always)]
    pub fn ma0l(&self) -> MA0L_R {
        MA0L_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA0L")
            .field("ma0l", &format_args!("{}", self.ma0l().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACA0L_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address0 low"]
    #[inline(always)]
    #[must_use]
    pub fn ma0l(&mut self) -> MA0L_W<MACA0L_SPEC> {
        MA0L_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC address 0 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA0L_SPEC;
impl crate::RegisterSpec for MACA0L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca0l::R`](R) reader structure"]
impl crate::Readable for MACA0L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca0l::W`](W) writer structure"]
impl crate::Writable for MACA0L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA0L to value 0xffff_ffff"]
impl crate::Resettable for MACA0L_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
