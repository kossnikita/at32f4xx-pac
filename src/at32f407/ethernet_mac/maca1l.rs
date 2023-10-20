#[doc = "Register `MACA1L` reader"]
pub type R = crate::R<MACA1L_SPEC>;
#[doc = "Register `MACA1L` writer"]
pub type W = crate::W<MACA1L_SPEC>;
#[doc = "Field `MA1L` reader - MAC address1 low"]
pub type MA1L_R = crate::FieldReader<u32>;
#[doc = "Field `MA1L` writer - MAC address1 low"]
pub type MA1L_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address1 low"]
    #[inline(always)]
    pub fn ma1l(&self) -> MA1L_R {
        MA1L_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA1L")
            .field("ma1l", &format_args!("{}", self.ma1l().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACA1L_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address1 low"]
    #[inline(always)]
    #[must_use]
    pub fn ma1l(&mut self) -> MA1L_W<MACA1L_SPEC> {
        MA1L_W::new(self, 0)
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
#[doc = "Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA1L_SPEC;
impl crate::RegisterSpec for MACA1L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca1l::R`](R) reader structure"]
impl crate::Readable for MACA1L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca1l::W`](W) writer structure"]
impl crate::Writable for MACA1L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA1L to value 0xffff_ffff"]
impl crate::Resettable for MACA1L_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
