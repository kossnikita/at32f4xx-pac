#[doc = "Register `MSTS` reader"]
pub type R = crate::R<MSTS_SPEC>;
#[doc = "Register `MSTS` writer"]
pub type W = crate::W<MSTS_SPEC>;
#[doc = "Field `FZC` reader - Freeze mode confirm"]
pub type FZC_R = crate::BitReader;
#[doc = "Field `DZC` reader - Doze mode confirm"]
pub type DZC_R = crate::BitReader;
#[doc = "Field `EOIF` reader - Error occur Interrupt flag"]
pub type EOIF_R = crate::BitReader;
#[doc = "Field `EOIF` writer - Error occur Interrupt flag"]
pub type EOIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QDZIF` reader - Quit doze mode interrupt flag"]
pub type QDZIF_R = crate::BitReader;
#[doc = "Field `QDZIF` writer - Quit doze mode interrupt flag"]
pub type QDZIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EDZIF` reader - Enter doze mode interrupt flag"]
pub type EDZIF_R = crate::BitReader;
#[doc = "Field `EDZIF` writer - Enter doze mode interrupt flag"]
pub type EDZIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CUSS` reader - Currently sending status"]
pub type CUSS_R = crate::BitReader;
#[doc = "Field `CURS` reader - Currently receiving status"]
pub type CURS_R = crate::BitReader;
#[doc = "Field `LSAMPRX` reader - Last sample level of RX pin"]
pub type LSAMPRX_R = crate::BitReader;
#[doc = "Field `REALRX` reader - Real time level of RX pin"]
pub type REALRX_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Freeze mode confirm"]
    #[inline(always)]
    pub fn fzc(&self) -> FZC_R {
        FZC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Doze mode confirm"]
    #[inline(always)]
    pub fn dzc(&self) -> DZC_R {
        DZC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error occur Interrupt flag"]
    #[inline(always)]
    pub fn eoif(&self) -> EOIF_R {
        EOIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quit doze mode interrupt flag"]
    #[inline(always)]
    pub fn qdzif(&self) -> QDZIF_R {
        QDZIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enter doze mode interrupt flag"]
    #[inline(always)]
    pub fn edzif(&self) -> EDZIF_R {
        EDZIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Currently sending status"]
    #[inline(always)]
    pub fn cuss(&self) -> CUSS_R {
        CUSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Currently receiving status"]
    #[inline(always)]
    pub fn curs(&self) -> CURS_R {
        CURS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Last sample level of RX pin"]
    #[inline(always)]
    pub fn lsamprx(&self) -> LSAMPRX_R {
        LSAMPRX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Real time level of RX pin"]
    #[inline(always)]
    pub fn realrx(&self) -> REALRX_R {
        REALRX_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Error occur Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoif(&mut self) -> EOIF_W<MSTS_SPEC, 2> {
        EOIF_W::new(self)
    }
    #[doc = "Bit 3 - Quit doze mode interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn qdzif(&mut self) -> QDZIF_W<MSTS_SPEC, 3> {
        QDZIF_W::new(self)
    }
    #[doc = "Bit 4 - Enter doze mode interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn edzif(&mut self) -> EDZIF_W<MSTS_SPEC, 4> {
        EDZIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Main status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSTS_SPEC;
impl crate::RegisterSpec for MSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msts::R`](R) reader structure"]
impl crate::Readable for MSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msts::W`](W) writer structure"]
impl crate::Writable for MSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTS to value 0x0c02"]
impl crate::Resettable for MSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c02;
}
