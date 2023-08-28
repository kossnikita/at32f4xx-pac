#[doc = "Register `HDRV` reader"]
pub type R = crate::R<HDRV_SPEC>;
#[doc = "Register `HDRV` writer"]
pub type W = crate::W<HDRV_SPEC>;
#[doc = "Field `HDRV0` reader - Port x driver bit y"]
pub type HDRV0_R = crate::BitReader;
#[doc = "Field `HDRV0` writer - Port x driver bit y"]
pub type HDRV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV1` reader - Port x driver bit y"]
pub type HDRV1_R = crate::BitReader;
#[doc = "Field `HDRV1` writer - Port x driver bit y"]
pub type HDRV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV2` reader - Port x driver bit y"]
pub type HDRV2_R = crate::BitReader;
#[doc = "Field `HDRV2` writer - Port x driver bit y"]
pub type HDRV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV3` reader - Port x driver bit y"]
pub type HDRV3_R = crate::BitReader;
#[doc = "Field `HDRV3` writer - Port x driver bit y"]
pub type HDRV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV4` reader - Port x driver bit y"]
pub type HDRV4_R = crate::BitReader;
#[doc = "Field `HDRV4` writer - Port x driver bit y"]
pub type HDRV4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV5` reader - Port x driver bit y"]
pub type HDRV5_R = crate::BitReader;
#[doc = "Field `HDRV5` writer - Port x driver bit y"]
pub type HDRV5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV6` reader - Port x driver bit y"]
pub type HDRV6_R = crate::BitReader;
#[doc = "Field `HDRV6` writer - Port x driver bit y"]
pub type HDRV6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV7` reader - Port x driver bit y"]
pub type HDRV7_R = crate::BitReader;
#[doc = "Field `HDRV7` writer - Port x driver bit y"]
pub type HDRV7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV8` reader - Port x driver bit y"]
pub type HDRV8_R = crate::BitReader;
#[doc = "Field `HDRV8` writer - Port x driver bit y"]
pub type HDRV8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV9` reader - Port x driver bit y"]
pub type HDRV9_R = crate::BitReader;
#[doc = "Field `HDRV9` writer - Port x driver bit y"]
pub type HDRV9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV10` reader - Port x driver bit y"]
pub type HDRV10_R = crate::BitReader;
#[doc = "Field `HDRV10` writer - Port x driver bit y"]
pub type HDRV10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV11` reader - Port x driver bit y"]
pub type HDRV11_R = crate::BitReader;
#[doc = "Field `HDRV11` writer - Port x driver bit y"]
pub type HDRV11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV12` reader - Port x driver bit y"]
pub type HDRV12_R = crate::BitReader;
#[doc = "Field `HDRV12` writer - Port x driver bit y"]
pub type HDRV12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV13` reader - Port x driver bit y"]
pub type HDRV13_R = crate::BitReader;
#[doc = "Field `HDRV13` writer - Port x driver bit y"]
pub type HDRV13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV14` reader - Port x driver bit y"]
pub type HDRV14_R = crate::BitReader;
#[doc = "Field `HDRV14` writer - Port x driver bit y"]
pub type HDRV14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRV15` reader - Port x driver bit y"]
pub type HDRV15_R = crate::BitReader;
#[doc = "Field `HDRV15` writer - Port x driver bit y"]
pub type HDRV15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv0(&self) -> HDRV0_R {
        HDRV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv1(&self) -> HDRV1_R {
        HDRV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv2(&self) -> HDRV2_R {
        HDRV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv3(&self) -> HDRV3_R {
        HDRV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv4(&self) -> HDRV4_R {
        HDRV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv5(&self) -> HDRV5_R {
        HDRV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv6(&self) -> HDRV6_R {
        HDRV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv7(&self) -> HDRV7_R {
        HDRV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv8(&self) -> HDRV8_R {
        HDRV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv9(&self) -> HDRV9_R {
        HDRV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv10(&self) -> HDRV10_R {
        HDRV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv11(&self) -> HDRV11_R {
        HDRV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv12(&self) -> HDRV12_R {
        HDRV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv13(&self) -> HDRV13_R {
        HDRV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv14(&self) -> HDRV14_R {
        HDRV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv15(&self) -> HDRV15_R {
        HDRV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv0(&mut self) -> HDRV0_W<HDRV_SPEC, 0> {
        HDRV0_W::new(self)
    }
    #[doc = "Bit 1 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv1(&mut self) -> HDRV1_W<HDRV_SPEC, 1> {
        HDRV1_W::new(self)
    }
    #[doc = "Bit 2 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv2(&mut self) -> HDRV2_W<HDRV_SPEC, 2> {
        HDRV2_W::new(self)
    }
    #[doc = "Bit 3 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv3(&mut self) -> HDRV3_W<HDRV_SPEC, 3> {
        HDRV3_W::new(self)
    }
    #[doc = "Bit 4 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv4(&mut self) -> HDRV4_W<HDRV_SPEC, 4> {
        HDRV4_W::new(self)
    }
    #[doc = "Bit 5 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv5(&mut self) -> HDRV5_W<HDRV_SPEC, 5> {
        HDRV5_W::new(self)
    }
    #[doc = "Bit 6 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv6(&mut self) -> HDRV6_W<HDRV_SPEC, 6> {
        HDRV6_W::new(self)
    }
    #[doc = "Bit 7 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv7(&mut self) -> HDRV7_W<HDRV_SPEC, 7> {
        HDRV7_W::new(self)
    }
    #[doc = "Bit 8 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv8(&mut self) -> HDRV8_W<HDRV_SPEC, 8> {
        HDRV8_W::new(self)
    }
    #[doc = "Bit 9 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv9(&mut self) -> HDRV9_W<HDRV_SPEC, 9> {
        HDRV9_W::new(self)
    }
    #[doc = "Bit 10 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv10(&mut self) -> HDRV10_W<HDRV_SPEC, 10> {
        HDRV10_W::new(self)
    }
    #[doc = "Bit 11 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv11(&mut self) -> HDRV11_W<HDRV_SPEC, 11> {
        HDRV11_W::new(self)
    }
    #[doc = "Bit 12 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv12(&mut self) -> HDRV12_W<HDRV_SPEC, 12> {
        HDRV12_W::new(self)
    }
    #[doc = "Bit 13 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv13(&mut self) -> HDRV13_W<HDRV_SPEC, 13> {
        HDRV13_W::new(self)
    }
    #[doc = "Bit 14 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv14(&mut self) -> HDRV14_W<HDRV_SPEC, 14> {
        HDRV14_W::new(self)
    }
    #[doc = "Bit 15 - Port x driver bit y"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv15(&mut self) -> HDRV15_W<HDRV_SPEC, 15> {
        HDRV15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Huge current driver\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdrv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdrv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDRV_SPEC;
impl crate::RegisterSpec for HDRV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdrv::R`](R) reader structure"]
impl crate::Readable for HDRV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hdrv::W`](W) writer structure"]
impl crate::Writable for HDRV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HDRV to value 0"]
impl crate::Resettable for HDRV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}