#[doc = "Register `BK1TMGWR4` reader"]
pub type R = crate::R<BK1TMGWR4_SPEC>;
#[doc = "Register `BK1TMGWR4` writer"]
pub type W = crate::W<BK1TMGWR4_SPEC>;
#[doc = "Field `ADDRST` reader - Address setup time"]
pub type ADDRST_R = crate::FieldReader;
#[doc = "Field `ADDRST` writer - Address setup time"]
pub type ADDRST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADDRHT` reader - Address-hold time"]
pub type ADDRHT_R = crate::FieldReader;
#[doc = "Field `ADDRHT` writer - Address-hold time"]
pub type ADDRHT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DTST` reader - Asynchronous data setup time"]
pub type DTST_R = crate::FieldReader;
#[doc = "Field `DTST` writer - Asynchronous data setup time"]
pub type DTST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `BUSLAT` reader - Bus latency"]
pub type BUSLAT_R = crate::FieldReader;
#[doc = "Field `BUSLAT` writer - Bus latency"]
pub type BUSLAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ASYNCM` reader - Asynchronous mode"]
pub type ASYNCM_R = crate::FieldReader;
#[doc = "Field `ASYNCM` writer - Asynchronous mode"]
pub type ASYNCM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn addrst(&self) -> ADDRST_R {
        ADDRST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold time"]
    #[inline(always)]
    pub fn addrht(&self) -> ADDRHT_R {
        ADDRHT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Asynchronous data setup time"]
    #[inline(always)]
    pub fn dtst(&self) -> DTST_R {
        DTST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&self) -> BUSLAT_R {
        BUSLAT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Asynchronous mode"]
    #[inline(always)]
    pub fn asyncm(&self) -> ASYNCM_R {
        ASYNCM_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    #[must_use]
    pub fn addrst(&mut self) -> ADDRST_W<BK1TMGWR4_SPEC, 0> {
        ADDRST_W::new(self)
    }
    #[doc = "Bits 4:7 - Address-hold time"]
    #[inline(always)]
    #[must_use]
    pub fn addrht(&mut self) -> ADDRHT_W<BK1TMGWR4_SPEC, 4> {
        ADDRHT_W::new(self)
    }
    #[doc = "Bits 8:15 - Asynchronous data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn dtst(&mut self) -> DTST_W<BK1TMGWR4_SPEC, 8> {
        DTST_W::new(self)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    #[must_use]
    pub fn buslat(&mut self) -> BUSLAT_W<BK1TMGWR4_SPEC, 16> {
        BUSLAT_W::new(self)
    }
    #[doc = "Bits 28:29 - Asynchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn asyncm(&mut self) -> ASYNCM_W<BK1TMGWR4_SPEC, 28> {
        ASYNCM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SRAM/NOR-Flash write timing registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK1TMGWR4_SPEC;
impl crate::RegisterSpec for BK1TMGWR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk1tmgwr4::R`](R) reader structure"]
impl crate::Readable for BK1TMGWR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk1tmgwr4::W`](W) writer structure"]
impl crate::Writable for BK1TMGWR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK1TMGWR4 to value 0x0fff_ffff"]
impl crate::Resettable for BK1TMGWR4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
