#[doc = "Register `MACA2L` reader"]
pub type R = crate::R<MACA2L_SPEC>;
#[doc = "Register `MACA2L` writer"]
pub type W = crate::W<MACA2L_SPEC>;
#[doc = "Field `MA2L` reader - MAC address2 low"]
pub type MA2L_R = crate::FieldReader<u32>;
#[doc = "Field `MA2L` writer - MAC address2 low"]
pub type MA2L_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - MAC address2 low"]
    #[inline(always)]
    pub fn ma2l(&self) -> MA2L_R {
        MA2L_R::new(self.bits & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA2L")
            .field("ma2l", &format_args!("{}", self.ma2l().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACA2L_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:30 - MAC address2 low"]
    #[inline(always)]
    #[must_use]
    pub fn ma2l(&mut self) -> MA2L_W<MACA2L_SPEC> {
        MA2L_W::new(self, 0)
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
#[doc = "Ethernet MAC address 2 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA2L_SPEC;
impl crate::RegisterSpec for MACA2L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca2l::R`](R) reader structure"]
impl crate::Readable for MACA2L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca2l::W`](W) writer structure"]
impl crate::Writable for MACA2L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA2L to value 0xffff_ffff"]
impl crate::Resettable for MACA2L_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
