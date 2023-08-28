#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `PERR` reader - Parity error"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `FERR` reader - Framing error"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `NERR` reader - Noise error"]
pub type NERR_R = crate::BitReader;
#[doc = "Field `ROERR` reader - Receiver overflow error"]
pub type ROERR_R = crate::BitReader;
#[doc = "Field `IDLEF` reader - IDLE flag"]
pub type IDLEF_R = crate::BitReader;
#[doc = "Field `RDBF` reader - Receive data buffer full"]
pub type RDBF_R = crate::BitReader;
#[doc = "Field `RDBF` writer - Receive data buffer full"]
pub type RDBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDC` reader - Transmit data complete"]
pub type TDC_R = crate::BitReader;
#[doc = "Field `TDC` writer - Transmit data complete"]
pub type TDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDBE` reader - Transmit data buffer empty"]
pub type TDBE_R = crate::BitReader;
#[doc = "Field `BFF` reader - Break frame flag"]
pub type BFF_R = crate::BitReader;
#[doc = "Field `BFF` writer - Break frame flag"]
pub type BFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTSCF` reader - CTS change flag"]
pub type CTSCF_R = crate::BitReader;
#[doc = "Field `CTSCF` writer - CTS change flag"]
pub type CTSCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTODF` reader - Reiceiver time out detection flag"]
pub type RTODF_R = crate::BitReader;
#[doc = "Field `RTODF` writer - Reiceiver time out detection flag"]
pub type RTODF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDF` reader - Character match detection flag"]
pub type CMDF_R = crate::BitReader;
#[doc = "Field `CMDF` writer - Character match detection flag"]
pub type CMDF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise error"]
    #[inline(always)]
    pub fn nerr(&self) -> NERR_R {
        NERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver overflow error"]
    #[inline(always)]
    pub fn roerr(&self) -> ROERR_R {
        ROERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE flag"]
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive data buffer full"]
    #[inline(always)]
    pub fn rdbf(&self) -> RDBF_R {
        RDBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit data complete"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty"]
    #[inline(always)]
    pub fn tdbe(&self) -> TDBE_R {
        TDBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Break frame flag"]
    #[inline(always)]
    pub fn bff(&self) -> BFF_R {
        BFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS change flag"]
    #[inline(always)]
    pub fn ctscf(&self) -> CTSCF_R {
        CTSCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Reiceiver time out detection flag"]
    #[inline(always)]
    pub fn rtodf(&self) -> RTODF_R {
        RTODF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - Character match detection flag"]
    #[inline(always)]
    pub fn cmdf(&self) -> CMDF_R {
        CMDF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Receive data buffer full"]
    #[inline(always)]
    #[must_use]
    pub fn rdbf(&mut self) -> RDBF_W<STS_SPEC, 5> {
        RDBF_W::new(self)
    }
    #[doc = "Bit 6 - Transmit data complete"]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TDC_W<STS_SPEC, 6> {
        TDC_W::new(self)
    }
    #[doc = "Bit 8 - Break frame flag"]
    #[inline(always)]
    #[must_use]
    pub fn bff(&mut self) -> BFF_W<STS_SPEC, 8> {
        BFF_W::new(self)
    }
    #[doc = "Bit 9 - CTS change flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CTSCF_W<STS_SPEC, 9> {
        CTSCF_W::new(self)
    }
    #[doc = "Bit 11 - Reiceiver time out detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtodf(&mut self) -> RTODF_W<STS_SPEC, 11> {
        RTODF_W::new(self)
    }
    #[doc = "Bit 17 - Character match detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmdf(&mut self) -> CMDF_W<STS_SPEC, 17> {
        CMDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0xc0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
