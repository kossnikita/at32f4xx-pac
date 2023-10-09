#[doc = "Register `INTSTS` reader"]
pub type R = crate::R<INTSTS_SPEC>;
#[doc = "Register `INTSTS` writer"]
pub type W = crate::W<INTSTS_SPEC>;
#[doc = "Field `EPT_NUM` reader - Endpoint number"]
pub type EPT_NUM_R = crate::FieldReader;
#[doc = "Field `EPT_NUM` writer - Endpoint number"]
pub type EPT_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `INOUT` reader - In/Out transaction"]
pub type INOUT_R = crate::BitReader;
#[doc = "Field `INOUT` writer - In/Out transaction"]
pub type INOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSOF` reader - Lost start of frame"]
pub type LSOF_R = crate::BitReader;
#[doc = "Field `LSOF` writer - Lost start of frame"]
pub type LSOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOF` reader - start of frame"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - start of frame"]
pub type SOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RST` reader - Bus reset"]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - Bus reset"]
pub type RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SP` reader - Bus suspend"]
pub type SP_R = crate::BitReader;
#[doc = "Field `SP` writer - Bus suspend"]
pub type SP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WK` reader - Wakeup"]
pub type WK_R = crate::BitReader;
#[doc = "Field `WK` writer - Wakeup"]
pub type WK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BE` reader - Bus error"]
pub type BE_R = crate::BitReader;
#[doc = "Field `BE` writer - Bus error"]
pub type BE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UCFOR` reader - USB core fifo overrun memory"]
pub type UCFOR_R = crate::BitReader;
#[doc = "Field `UCFOR` writer - USB core fifo overrun memory"]
pub type UCFOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC` reader - transaction completed"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TC` writer - transaction completed"]
pub type TC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    pub fn ept_num(&self) -> EPT_NUM_R {
        EPT_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - In/Out transaction"]
    #[inline(always)]
    pub fn inout(&self) -> INOUT_R {
        INOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Lost start of frame"]
    #[inline(always)]
    pub fn lsof(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus suspend"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wk(&self) -> WK_R {
        WK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bus error"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USB core fifo overrun memory"]
    #[inline(always)]
    pub fn ucfor(&self) -> UCFOR_R {
        UCFOR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - transaction completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTS")
            .field("ept_num", &format_args!("{}", self.ept_num().bits()))
            .field("inout", &format_args!("{}", self.inout().bit()))
            .field("lsof", &format_args!("{}", self.lsof().bit()))
            .field("sof", &format_args!("{}", self.sof().bit()))
            .field("rst", &format_args!("{}", self.rst().bit()))
            .field("sp", &format_args!("{}", self.sp().bit()))
            .field("wk", &format_args!("{}", self.wk().bit()))
            .field("be", &format_args!("{}", self.be().bit()))
            .field("ucfor", &format_args!("{}", self.ucfor().bit()))
            .field("tc", &format_args!("{}", self.tc().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<INTSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    #[must_use]
    pub fn ept_num(&mut self) -> EPT_NUM_W<INTSTS_SPEC, 0> {
        EPT_NUM_W::new(self)
    }
    #[doc = "Bit 4 - In/Out transaction"]
    #[inline(always)]
    #[must_use]
    pub fn inout(&mut self) -> INOUT_W<INTSTS_SPEC, 4> {
        INOUT_W::new(self)
    }
    #[doc = "Bit 8 - Lost start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn lsof(&mut self) -> LSOF_W<INTSTS_SPEC, 8> {
        LSOF_W::new(self)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<INTSTS_SPEC, 9> {
        SOF_W::new(self)
    }
    #[doc = "Bit 10 - Bus reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<INTSTS_SPEC, 10> {
        RST_W::new(self)
    }
    #[doc = "Bit 11 - Bus suspend"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<INTSTS_SPEC, 11> {
        SP_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn wk(&mut self) -> WK_W<INTSTS_SPEC, 12> {
        WK_W::new(self)
    }
    #[doc = "Bit 13 - Bus error"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<INTSTS_SPEC, 13> {
        BE_W::new(self)
    }
    #[doc = "Bit 14 - USB core fifo overrun memory"]
    #[inline(always)]
    #[must_use]
    pub fn ucfor(&mut self) -> UCFOR_W<INTSTS_SPEC, 14> {
        UCFOR_W::new(self)
    }
    #[doc = "Bit 15 - transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<INTSTS_SPEC, 15> {
        TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTS_SPEC;
impl crate::RegisterSpec for INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsts::R`](R) reader structure"]
impl crate::Readable for INTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intsts::W`](W) writer structure"]
impl crate::Writable for INTSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
